use kanshi_rs::kanshi::Kanshi;
use kanshi_rs::script;
fn main() {
    let kanshi = Kanshi::new("/root/test.pid");
    kanshi.every(3).lazy_exec(|| script(""));
}
