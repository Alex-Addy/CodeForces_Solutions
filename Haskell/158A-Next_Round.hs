main = do
	l1 <- getLine
	l2 <- getLine
	let k = read $ words l1 !! 1 in	putStrLn $ show $ solve k $ map (read) (words l2)
	
solve :: Int -> [Int] -> Int
solve k scores = length [x | x <- scores, x >= scores !! (k-1), x > 0]