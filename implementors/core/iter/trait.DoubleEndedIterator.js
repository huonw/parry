(function() {var implementors = {};
implementors['libc'] = [];implementors['num'] = ["impl&lt;A&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/iter/trait.DoubleEndedIterator.html' title='core::iter::DoubleEndedIterator'>DoubleEndedIterator</a> for <a class='struct' href='num/iter/struct.Range.html' title='num::iter::Range'>Range</a>&lt;A&gt; <span class='where'>where A: <a class='trait' href='num/integer/trait.Integer.html' title='num::integer::Integer'>Integer</a> + <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html' title='core::cmp::PartialOrd'>PartialOrd</a> + <a class='trait' href='https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> + <a class='trait' href='num/traits/trait.ToPrimitive.html' title='num::traits::ToPrimitive'>ToPrimitive</a></span>","impl&lt;A&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/iter/trait.DoubleEndedIterator.html' title='core::iter::DoubleEndedIterator'>DoubleEndedIterator</a> for <a class='struct' href='num/iter/struct.RangeInclusive.html' title='num::iter::RangeInclusive'>RangeInclusive</a>&lt;A&gt; <span class='where'>where A: <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Sub.html' title='core::ops::Sub'>Sub</a>&lt;A, Output=A&gt; + <a class='trait' href='num/integer/trait.Integer.html' title='num::integer::Integer'>Integer</a> + <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html' title='core::cmp::PartialOrd'>PartialOrd</a> + <a class='trait' href='https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> + <a class='trait' href='num/traits/trait.ToPrimitive.html' title='num::traits::ToPrimitive'>ToPrimitive</a></span>",];implementors['parry'] = ["impl&lt;Op, X&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/iter/trait.DoubleEndedIterator.html' title='core::iter::DoubleEndedIterator'>DoubleEndedIterator</a> for <a class='struct' href='parry/iterators/struct.Unary.html' title='parry::iterators::Unary'>Unary</a>&lt;Op, X&gt; <span class='where'>where Op: <a class='trait' href='parry/iterators/trait.UnOp.html' title='parry::iterators::UnOp'>UnOp</a>&lt;X::Item&gt;, X: <a class='trait' href='https://doc.rust-lang.org/nightly/core/iter/trait.DoubleEndedIterator.html' title='core::iter::DoubleEndedIterator'>DoubleEndedIterator</a></span>","impl&lt;Op, X, Y&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/iter/trait.DoubleEndedIterator.html' title='core::iter::DoubleEndedIterator'>DoubleEndedIterator</a> for <a class='struct' href='parry/iterators/struct.Binary.html' title='parry::iterators::Binary'>Binary</a>&lt;Op, X, Y&gt; <span class='where'>where Op: <a class='trait' href='parry/iterators/trait.BinOp.html' title='parry::iterators::BinOp'>BinOp</a>&lt;X::Item, Y::Item&gt;, X: <a class='trait' href='https://doc.rust-lang.org/nightly/core/iter/trait.DoubleEndedIterator.html' title='core::iter::DoubleEndedIterator'>DoubleEndedIterator</a>, Y: <a class='trait' href='https://doc.rust-lang.org/nightly/core/iter/trait.DoubleEndedIterator.html' title='core::iter::DoubleEndedIterator'>DoubleEndedIterator</a></span>","impl&lt;B, T, E&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/iter/trait.DoubleEndedIterator.html' title='core::iter::DoubleEndedIterator'>DoubleEndedIterator</a> for <a class='struct' href='parry/iterators/struct.SwitchIter.html' title='parry::iterators::SwitchIter'>SwitchIter</a>&lt;B, T, E&gt; <span class='where'>where B: <a class='trait' href='https://doc.rust-lang.org/nightly/core/iter/trait.DoubleEndedIterator.html' title='core::iter::DoubleEndedIterator'>DoubleEndedIterator</a>&lt;Item=<a href='https://doc.rust-lang.org/nightly/std/primitive.bool.html'>bool</a>&gt;, T: <a class='trait' href='https://doc.rust-lang.org/nightly/core/iter/trait.DoubleEndedIterator.html' title='core::iter::DoubleEndedIterator'>DoubleEndedIterator</a>, E: <a class='trait' href='https://doc.rust-lang.org/nightly/core/iter/trait.DoubleEndedIterator.html' title='core::iter::DoubleEndedIterator'>DoubleEndedIterator</a>&lt;Item=T::Item&gt;</span>",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
