extern crate argh;
extern crate win32;
mod logging;
use anyhow::anyhow;
use std::{
    cell::RefCell,
    collections::HashSet,
    io::{Read, Seek, Write},
    path::{Path, PathBuf},
    rc::Rc,
};

#[cfg(feature = "sdl")]
mod sdl;
#[cfg(feature = "sdl")]
use sdl::GUI;
#[cfg(not(feature = "sdl"))]
mod headless;
#[cfg(not(feature = "sdl"))]
use headless::GUI;

// Reserved area: pagezero is 0x1000, we want to reserve 4gb-0x1000,
// but experimentally if I use constants larger than the below the resulting macho file
// has a section sized zero, even though wine uses a larger constant in similar code (?!).
// Possibly related to
// https://github.com/llvm/llvm-project/commit/b822063669641570ab5edae72956d18a5bcde8c4
// somehow?
std::arch::global_asm!(".zerofill RESV32,RESV32,__retrowin32_reserve,0x7f000000-0x1000");
std::arch::global_asm!(".no_dead_strip __retrowin32_reserve");

#[cfg(feature = "cpuemu")]
fn dump_asm(machine: &win32::Machine) {
    let instrs = win32::disassemble(machine.mem(), machine.x86.cpu.regs.eip);

    for instr in &instrs[..std::cmp::min(instrs.len(), 5)] {
        print!("{:08x} {:10} ", instr.addr, instr.bytes);
        for part in &instr.code {
            print!("{}", part.text);
        }
        println!();
    }
}

struct File {
    f: std::fs::File,
}
impl File {
    fn open(path: &Path) -> Self {
        let f = match std::fs::File::open(path) {
            Ok(f) => f,
            Err(err) => {
                log::error!("opening {:?}: {}", path, err);
                std::fs::File::open("/dev/null").unwrap()
            }
        };
        File { f }
    }
}
impl win32::File for File {
    fn seek(&mut self, ofs: u32) -> bool {
        self.f.seek(std::io::SeekFrom::Start(ofs as u64)).unwrap();
        true
    }

    fn read(&mut self, buf: &mut [u8], len: &mut u32) -> bool {
        let n = self.f.read(buf).unwrap();
        *len = n as u32;
        true
    }
}

struct Env {
    gui: Option<GUI>,
    exit_code: Option<u32>,
    cwd: PathBuf,
}

impl Env {
    pub fn new(cwd: PathBuf) -> Self {
        Env {
            gui: None,
            exit_code: None,
            cwd,
        }
    }

    pub fn ensure_gui(&mut self) -> anyhow::Result<&mut GUI> {
        if self.gui.is_none() {
            self.gui = Some(GUI::new()?);
        }
        Ok(self.gui.as_mut().unwrap())
    }
}

#[derive(Clone)]
struct EnvRef(Rc<RefCell<Env>>);

impl win32::Host for EnvRef {
    fn exit(&mut self, code: u32) {
        self.0.borrow_mut().exit_code = Some(code);
    }

    fn time(&self) -> u32 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u32
    }

    fn open(&self, path: &str) -> Box<dyn win32::File> {
        let env = self.0.borrow();
        Box::new(File::open(&env.cwd.join(path)))
    }

    fn write(&self, buf: &[u8]) -> usize {
        std::io::stdout().lock().write(buf).unwrap()
    }

    fn create_window(&mut self) -> Box<dyn win32::Window> {
        let mut env = self.0.borrow_mut();
        let gui = env.ensure_gui().unwrap();
        gui.create_window()
    }

    fn create_surface(&mut self, opts: &win32::SurfaceOptions) -> Box<dyn win32::Surface> {
        let mut env = self.0.borrow_mut();
        let gui = env.ensure_gui().unwrap();
        gui.create_surface(opts)
    }
}

fn hex_arg(arg: &str) -> Result<u32, String> {
    u32::from_str_radix(arg, 16).map_err(|err| err.to_string())
}

#[derive(argh::FromArgs)]
/// win32 emulator.
struct Args {
    /// win32 modules to trace calls into
    #[argh(option)]
    win32_trace: Option<String>,

    #[argh(option, from_str_fn(hex_arg))]
    /// addresses to dump emulator state
    trace_points: Vec<u32>,

    /// exe to run
    #[argh(positional)]
    exe: String,

    /// cmdline to pass to exe
    #[argh(positional)]
    cmdline: Option<String>,
}

fn main() -> anyhow::Result<()> {
    logging::init()?;
    let args: Args = argh::from_env();
    win32::trace::set_scheme(args.win32_trace.as_deref().unwrap_or("-"));
    let cmdline = args.cmdline.as_ref().unwrap_or(&args.exe);

    let buf = std::fs::read(&args.exe).map_err(|err| anyhow!("{}: {}", args.exe, err))?;
    let cwd = Path::parent(Path::new(&args.exe)).unwrap();
    let host = EnvRef(Rc::new(RefCell::new(Env::new(cwd.to_owned()))));
    let mut machine = win32::Machine::new(Box::new(host.clone()));

    println!("rust main fn at {:x}", main as u64);

    let mp: *const win32::Machine = &machine;
    println!("rust stack var at {:x}", mp as u64);

    let hr = Box::new(3);
    let hp: *const i32 = hr.as_ref();
    println!("rust heap var at {:x}", hp as u64);

    let mut sbuf = String::new();
    std::io::stdin().read_line(&mut sbuf).unwrap();

    // println!("{}", VAR1[0]);

    let entry_point = machine
        .load_exe(&buf, cmdline.clone(), false)
        .map_err(|err| anyhow!("loading {}: {}", args.exe, err))?;
    #[cfg(not(feature = "cpuemu"))]
    {
        let seg: u32 = 32;
        let seg_selector: u32 = (seg << 3) | 0b111;
        let m1632: u64 = ((seg_selector as u64) << 32) | entry_point as u64;
        println!("entry point at {:x}, about to jump", entry_point);
        //let go: extern "C" fn() = unsafe { std::mem::transmute(entry_point as u64) };
        std::io::stdin().read_line(&mut sbuf).unwrap();
        println!("targ {:x}", m1632);
        println!("targaddr {:x}", &m1632 as *const u64 as u64);
        //go();
        unsafe {
            std::arch::asm!(
                //"mov fs,[{teb}]",
                "lcall [{ep}]",
                //teb = in(reg) machine.state.kernel32.teb,
                ep = in(reg) &m1632,
            );
        }
    }

    #[cfg(feature = "cpuemu")]
    {
        let mut trace_points = HashSet::new();
        for &tp in &args.trace_points {
            trace_points.insert(tp);
            machine.x86.add_breakpoint(machine.memory.mem(), tp);
        }

        let start = std::time::Instant::now();
        loop {
            if let Some(gui) = &mut host.0.borrow_mut().gui {
                if !gui.pump_messages() {
                    break;
                }
            }
            match machine.execute_block() {
                Err(err) => {
                    dump_asm(&machine);
                    log::error!("{:?}", err);
                    break;
                }
                Ok(done) => {
                    if host.0.borrow().exit_code.is_some() {
                        break;
                    }

                    let ip = machine.x86.cpu.regs.eip;
                    if !done && trace_points.contains(&ip) {
                        let regs = &machine.x86.cpu.regs;
                        eprintln!(
                            "trace ip:{:x} eax:{:x} ebx:{:x} ecx:{:x} edx:{:x} esi:{:x} edi:{:x}",
                            regs.eip, regs.eax, regs.ebx, regs.ecx, regs.edx, regs.esi, regs.edi
                        );
                        machine.x86.clear_breakpoint(machine.memory.mem(), ip);
                        machine.single_step().unwrap();
                        machine.x86.add_breakpoint(machine.memory.mem(), ip);
                    }
                }
            }
        }
        let millis = start.elapsed().as_millis() as usize;
        if millis > 0 {
            eprintln!(
                "{} instrs in {} ms: {}m/s",
                machine.x86.instr_count,
                millis,
                (machine.x86.instr_count / millis) / 1000
            );
        }
    }

    Ok(())
}
