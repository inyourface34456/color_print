pub fn max<T: std::cmp::PartialOrd>(v1: T, v2: T, v3: T) -> T {
    if v1 == v2 {
        if v1 > v3 {
            v1
        } else {
            v2
        }
    } else if v2 == v3 {
        if v2 > v1 {
            v2
        } else {
            v1
        }
    } else if v1 == v3 {
        if v1 > v2 {
            v1
        } else {
            v2
        }
    } else if v1 > v2 && v1 > v3 {
        v1
    } else if v2 > v3 && v2 > v1 {
        v2
    } else {
        v3
    }
}

pub fn min<T: std::cmp::PartialOrd>(v1: T, v2: T, v3: T) -> T {
    if v1 == v2 {
        if v1 < v3 {
            v1
        } else {
            v3
        }
    } else if v2 == v3 {
        if v2 < v1 {
            v2
        } else {
            v1
        }
    } else if v1 == v3 {
        if v1 < v2 {
            v1
        } else {
            v2
        }
    } else if v1 < v2 && v1 < v3 {
        v1
    } else if v2 < v3 && v2 < v1 {
        v2
    } else {
        v3
    }
}
