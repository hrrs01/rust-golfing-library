fn main(){
    let x = ||{let mut y=29;for z in 1..20{println!("{0}{1}{2}{2}{1}{0}","|".repeat(29-y),"~","|".repeat(y));y-=1}y};
    x();
}
