if' :: Bool -> a -> a -> a
if' True x _ = x
if' False _ y = y

main = do
	s <- getLine
	putStrLn $ if' (read s `mod` 2 == 0 && read s /= 2) "YES" "NO"