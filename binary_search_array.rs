fn binary_search(target: i32, a: &[i32; 10]) -> bool {
    let mut l: usize = 0;
    let mut r: usize = a.len() - 1;

    while l <= r {
        let mid: usize = l + (r - l) / 2;

        if a[mid] == target {
            return true;
        }
        if a[mid] < target {
            l = mid + 1;
        } else if r > 0 {
            r = mid - 1;
        } else {
            return false;
        }
    }
    if l > 0 && a[l - 1] == target {
        return true;
    }

    return false;
}

fn bisect_left(target: i32, a: &[i32; 10]) -> usize {
    let mut l: usize = 0;
    let mut r: usize = a.len() - 1;

    while l < r {
        let mid: usize = l + (r - l) / 2;

        if a[mid] < target {
            l = mid + 1;
        } else {
            r = mid;
        }
    }
    return l;
}

fn bisect_right(target: i32, a: &[i32; 10]) -> usize {
    let mut l: usize = 0;
    let mut r: usize = a.len() - 1;

    while l < r {
        let mid: usize = l + (r - l) / 2;

        if a[mid] <= target {
            l = mid + 1;
        } else {
            r = mid;
        }
    }
    return l;
}

fn main() {
    let b: [i32; 10] = [1, 2, 5, 7, 11, 15, 18, 21, 36, 42];
    let a: [i32; 10] = [0, 42, 42, 42, 42, 69, 69, 69, 69, 69];
    let target: i32 = 42;

    let found: bool = binary_search(target, &b);

    println!("Binary Search");
    println!("Was Found:");
    println!("{}", found);

    let mut i: usize = bisect_left(target, &a);

    println!("Bisect Left");
    println!("Target Index:");
    println!("{}", i);
    i = bisect_right(target, &a);
    println!("Bisect Right");

    println!("Target Index:");
    println!("{}", i);
}
