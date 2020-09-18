use super::SortOrder;

pub fn sort<T:Ord>(x:&mut [T],order:&SortOrder){
    match *order{
        SortOrder::Ascending=>do_sort(x,true),
        SortOrder::Descending=>do_sort(x,false),
    };
}

fn do_sort<T:Ord>(x:&mut [T],up:bool){
    if x.len()>1{
        let mid_point=x.len()/2;
        do_sort(&mut x[..mid_point],true);
        do_sort(&mut x[mid_point..],false);
        sub_sort(x,up);
    }
}

fn sub_sort<T:Ord>(x:&mut [T], up:bool){
    if x.len()>1{
        compare_and_swap(x,up);
        let mid_point=x.len()/2;
        sub_sort(&mut x[..mid_point],up);
        sub_sort(&mut x[mid_point..],up);
    }
}

fn compare_and_swap<T:Ord>(x:&mut [T],up:bool){
    let mid_point=x.len()/2;
    for i in 0..mid_point{
        if(x[i]>x[mid_point+i])==up{
            x.swap(i,mid_point+i);
        }
    }
}


#[cfg(test)]

mod tests{
    use super::sort;
    use crate::SortOrder::*;
    #[test]
    fn sort_u32_ascending(){
        let mut x=vec!["Rust","is","fast","and","memory-efficient","with","no","GC"];

        sort(&mut x,&Ascending);
        assert_eq!(x,vec!["GC","Rust","and","fast","is","memory-efficient","no","with"]);
    }
    #[test]
    fn sort_u32_descending(){
        let mut x=vec!["Rust","is","fast","and","memory-efficient","with","no","GC"];
        sort(&mut x,&Descending);
        assert_eq!(x,vec!["with","no","memory-efficient","is","fast","and","Rust","GC"]);
    }
}