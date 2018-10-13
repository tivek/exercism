pub fn map<V, F, I, O>(input: V, mut _function: F) -> Vec<O>
where
    V: IntoIterator<Item = I>,
    F: FnMut(I) -> O,
{
    let mut out = Vec::new();
    for i in input {
        out.push(_function(i));
    }
    out
}
