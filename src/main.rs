mod daily;
use daily::all_solutions;

fn main() {
    let solutions = all_solutions();

    for (date, solution) in solutions {
        let result = solution();
        println!("{}: {:#?}", date, result);
    }
}
