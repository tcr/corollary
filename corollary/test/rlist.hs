module Test()
where
import Prelude hiding (reverse)
import qualified Data.List as List

newtype Reversed a = Reversed a
type RList a = Reversed [a]
empty :: Reversed [a]
empty = Reversed []

singleton :: a -> Reversed [a]
singleton x = Reversed [x]

-- snoc :: Reversed [a] -> a -> Reversed [a]
-- snoc (Reversed xs) x = Reversed (x : xs)
-- infixr 5 `snoc`

rappend :: Reversed [a] -> [a] -> Reversed [a]
rappend (Reversed xs) ys = Reversed (List.reverse ys ++ xs)

appendr :: [a] -> Reversed [a] -> Reversed [a]
appendr xs (Reversed ys) = Reversed (ys ++ List.reverse xs)

rappendr :: Reversed [a] -> Reversed [a] -> Reversed [a]
rappendr (Reversed xs) (Reversed ys) = Reversed (ys ++ xs)

rmap :: (a -> b) -> Reversed [a] -> Reversed [b]
rmap f (Reversed xs) = Reversed (map f xs)

reverse :: Reversed [a] -> [a]
reverse (Reversed xs) = List.reverse xs

inner :: Reversed [a] -> [a]
inner (Reversed xs) = xs

-- viewr :: Reversed [a] -> (Reversed [a] , a)
-- viewr (Reversed []) = error "viewr: empty RList"
-- viewr (Reversed (x:xs)) = (Reversed xs, x)

main :: IO ()
main = do
    let a = singleton "hello world"
    putStrLn ((inner a) !! 0)
