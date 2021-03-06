use dvec::DVec;

type entry<A,B> = {key: A, value: B};
type alist<A,B> = { eq_fn: fn@(A,A) -> bool, data: DVec<entry<A,B>> };

fn alist_add<A: Copy, B: Copy>(lst: alist<A,B>, k: A, v: B) {
    lst.data.push({key:k, value:v});
}

fn alist_get<A: Copy, B: Copy>(lst: alist<A,B>, k: A) -> B {
    let eq_fn = lst.eq_fn;
    for lst.data.each |entry| {
        if eq_fn(entry.key, k) { return entry.value; }
    }
    fail;
}

#[inline]
fn new_int_alist<B: Copy>() -> alist<int, B> {
    fn eq_int(&&a: int, &&b: int) -> bool { a == b }
    return {eq_fn: eq_int, data: DVec()};
}

#[inline]
fn new_int_alist_2<B: Copy>() -> alist<int, B> {
    #[inline]
    fn eq_int(&&a: int, &&b: int) -> bool { a == b }
    return {eq_fn: eq_int, data: DVec()};
}