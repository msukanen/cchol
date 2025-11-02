use std::fmt::Display;

pub trait NaturalJoin {
    fn natural_join(self) -> String;
}

impl<I> NaturalJoin for I
where
    I: Iterator,
    I::Item: Display,
{
    /// Join stuff:
    /// 
    /// ```
    /// use rpgassist::ext::natural_join::NaturalJoin;
    /// // results with: s = "A"
    /// let v = ["A"];
    /// let s = v.iter().natural_join();
    /// 
    /// // results in: s = "A and B"
    /// let v = ["A", "B"];
    /// let s = v.iter().natural_join();
    /// 
    /// // results in s = "A, B and C"
    /// let v = ["A", "B", "C"];
    /// let s = v.iter().natural_join();
    /// ```
    fn natural_join(self) -> String {
        // check if more than one entry…
        let mut iter = self.peekable();
        let mut output = String::new();

        // 1st item, if any…
        match iter.next() {
            None => return output,
            Some(fst) => {
                // break early if just single entry
                if iter.peek().is_none() {
                    return fst.to_string();
                }
                output.push_str(&fst.to_string())
            },
        }

        while let Some(curr) = iter.next() {
            if iter.peek().is_none() {
                output.push_str(" and ");
            } else {
                output.push_str(", ");
            }
            output.push_str(&curr.to_string());
        }

        output
    }
}