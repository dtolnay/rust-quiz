Answer: 21
Difficulty: 1

# Hint

Does `s` get moved?

# Explanation

The relevant line is `let _  = s`. If this line does not move `s` then `s` will
continue to live until the close curly brace and the program would print `21`.
But if this line does move `s`, without binding it, then the moved value of type
`S` would be dropped immediately and the program would print `12`.

In fact `s` does not get moved and the output is `21`.
