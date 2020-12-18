import Prelude(print, sum, ($))
import qualified Prelude

infixr 0 *
(*) = (Prelude.*)
infixr 1 +
(+) = (Prelude.+)

main = print $ sum [
                    2 * 3 + 5
                   ,2 * 3 + (4 * 5)
                   ,5 + (8 * 3 + 9 + 3 * 4 * 3)
                   ,5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
                   ,((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2
                   ]
