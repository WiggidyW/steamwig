fn main() {
    let s: String = powershell_script::run(r"Import-Module C:\Users\user\Programming\steamwig\assets\AudioDeviceCmdlets.dll; Get-AudioDevice -Playback").unwrap().stdout().unwrap();

    // println!("{:?}", s);

    let r: regex::Regex = regex::Regex::new(r"\{\d\.\d\.\d\.(\d{8})\}\.\{([a-z]|\d){8}-([a-z]|\d){4}-([a-z]|\d){4}-([a-z]|\d){4}-([a-z]|\d){12}\}").unwrap();
    let m = r.find(&s).unwrap().as_str();
    println!("{:?}", m);
}