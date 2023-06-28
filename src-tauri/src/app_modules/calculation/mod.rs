#[tauri::command]
pub fn run_calculation(p1: f64, p2: f64, t1: f64, t2: f64){
    println!("called");
    println!("{}", p1);
    println!("{}", p2);
    println!("{}", t1);
    println!("{}", t2);
}