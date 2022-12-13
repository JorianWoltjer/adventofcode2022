use day7::Shell;

fn main() {
    let input = include_str!("../../input.txt");

    let mut shell: Shell = input.parse().unwrap();
    shell.change_directory("/").unwrap();

    let root = shell.levels.front().unwrap().borrow();

    let total_under_100000: u64 = root.get_all_sizes().iter().filter(|size| size <= &&100_000).sum();

    println!("Total under: {}", total_under_100000);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        let mut shell: Shell = input.parse().unwrap();
        shell.change_directory("/").unwrap();

        let shell = dbg!(shell);

        let root = shell.levels.front().unwrap().borrow();

        let total_size = root.get_total_size();
        let mut all_sizes = root.get_all_sizes();
        assert_eq!(total_size, 48381165);
        all_sizes.sort();
        assert_eq!(all_sizes, vec![584, 94853, 24933642, 48381165]);

        let total_under_100000: u64 = root.get_all_sizes().iter().filter(|size| size <= &&100_000).sum();

        assert_eq!(total_under_100000, 95437);
    }
}
