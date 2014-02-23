
if' :: Bool -> a -> a -> a
if' True  x _ = x
if' False _ y = y

main = do
    program <- getLine
    putStrLn $  if' (willOutput program) "YES" "NO"

willOutput :: String -> Bool
willOutput s  = not $ null [ c | c <- s, c `elem` "HQ9" ]

