import Data.Char

if' :: Bool -> a -> a -> a
if' True  x _ = x
if' False _ y = y

main = do
    word <- getLine
    putStrLn $ fixCaps word

fixCaps :: String -> String
fixCaps s@(x:xs) = if' (foldl (\x y -> x && (isUpper y)) True s) (map toLower s) $
            		if' (foldl (\x y -> x && (isUpper y)) (isLower $ x) (xs)) ([toUpper $ x] ++ (map toLower $ xs)) $
            		s
