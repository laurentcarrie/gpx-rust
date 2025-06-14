use std::collections::HashSet;
use log::LevelFilter;
use regex::Regex;
use simple_logger::SimpleLogger;
use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;

fn check_summary(root:&PathBuf,mdfiles:&Vec<PathBuf>) -> bool {
    let mut path = root.clone() ;
    path.push("SUMMARY.md") ;
    let data = std::fs::read_to_string(&path) ;
    match data {
        Err(e) => {
            log::error!("could not read SUMMARY.md, {:?}",e) ;
            false
        },
        Ok(data) => {
            let re = Regex::new(r"\((.*?)\)").unwrap();
            let mut files_declared_in_summary: HashSet<PathBuf> = HashSet::new() ;
            for (_, [target_file]) in re.captures_iter(&data).map(|c| c.extract()) {
                let mut p = root.clone() ;
                p.push(target_file) ;
                files_declared_in_summary.insert(p);
            };
            let mut files_on_disk:HashSet<PathBuf>=HashSet::new() ;
            for p in mdfiles {
                if p.as_path().file_name().unwrap().to_str() != Some("SUMMARY.md") {
                    files_on_disk.insert(p.clone());
                }
            } ;
            let mut ok=true ;
            for  p in files_on_disk.difference(&files_declared_in_summary) {
                ok=false ;
                log::error!("[SUMMARY.md] {:?} on disk but not in SUMMARY.md",p) ;
            };
            ok
        } }


}

fn walk(path: &Path) -> Vec<PathBuf> {
    let mut acc: Vec<PathBuf> = vec![];
    // log::info!("ENTER {:?}",&path) ;
    for p in path.read_dir().expect("read dir failed") {
        if let Ok(p) = p {
            if let Ok(file_type) = p.file_type() {
                if file_type.is_dir() {
                    let mut other = walk(p.path().as_path());
                    acc.append(&mut other);
                } else if file_type.is_file() {
                    if p.path().extension() == Some(OsStr::new("md")) {
                        // log::info!("... file {:?}",&p);
                        acc.push(p.path());
                    }
                }
            }
        }
    }
    acc
}

fn absolute_path_of_link(root:&PathBuf,local:&PathBuf,link:&PathBuf) -> PathBuf {
    log::info!("APL root : {:?}",&root) ;
    log::info!("APL local : {:?}",&local) ;
    log::info!("APL link : {:?}",&link) ;
    // relative links need to be taken from relative to local
    // absolute links need to be modified and taken from root
   let p = if link.as_path().is_absolute() {
            log::info!("absolute path : {:?}",&link) ;
            let mut spath = link.to_str().unwrap().to_string();
            if spath.len() > 0 {
                spath.remove(0);
            };
            log::info!("spath is {:?}",&spath) ;
            let mut p = root.clone() ;
            p.push(spath) ;
            log::info!("path is now : {:?}",&p) ;
        p
    } else {
        log::info!("relative path : {:?}",&link) ;
        let mut p = local.clone() ;
        p.push(link) ;
        p
    } ;
    log::info!("return : {:?}",&p) ;
    p
}

fn check_reference(target_file: &PathBuf, target_link: &str) -> bool {
    log::info!("check reference, target file : {:?}",target_file) ;
    assert!(target_file.as_path().is_absolute()) ;
    let data = std::fs::read_to_string(&target_file);
    if let Ok(data) = data {
        let re = format!(r###"<a id="{}"/>"###, target_link);
        // log::info!("re : {}",&re) ;
        let re = Regex::new(&re).unwrap();
        match re.find(&data) {
            Some(_) => true,
            None => false,
        }
    } else {
        log::error!("error while reading {:?}", &target_file);
        false
    }
}

fn check_md_file(root:&PathBuf,p: &PathBuf) -> bool {
    let mut ok = true;
    log::info!("check file {:?}",&p) ;
    let data = std::fs::read_to_string(&p).expect("read file");
    // (render.md#sections)
    let re = Regex::new(r"\((.*?)#(.*?)\)").unwrap();
    // let re = Regex::new(r"\\((.*?)\\)").unwrap();
    // let mut results : Vec<String> = vec![];
    for (_, [target_file, target_link]) in re.captures_iter(&data).map(|c| c.extract()) {
        // log::info!("{:?} {:?}",target_file,target_link) ;
        let target_path = match target_file {
            "" => p.clone(),
            s => {
                log::info!("s is {:?}",s) ;
                let local = p.as_path().parent().unwrap().to_path_buf() ;
                let s = PathBuf::from(s) ;
                let link = absolute_path_of_link(&root,&local,&s) ;
                link
            }
        };
        let this_ok = check_reference(&target_path, &target_link);
        if !this_ok {
            log::error!(
                "in {:?}, unresolved link : {:?} {:?}",
                &p,
                &target_path,
                &target_link
            )
        }
        ok = ok && this_ok;
    }
    ok
}

fn main() {
    let _ = SimpleLogger::new().init().unwrap();
    log::set_max_level(LevelFilter::Error);

    let mut args: std::env::Args = env::args();
    log::info!("found {} args on command line", args.len());
    let root = PathBuf::from(args.nth(1).unwrap());
    let mdfiles = walk(&root.as_path());
    let mut ok = true;
    for p in &mdfiles {
        // ok = ok && check_md_file(root.clone(),p);
        let x = check_md_file(&root,&p);
        ok = ok && x ;
    } ;
    let ok = check_summary(&root,&mdfiles) ;
    if !ok {
        log::info!("done, exiting with code 1") ;
        exit(1)
    }
}
