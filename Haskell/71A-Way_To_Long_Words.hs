
main = do
		n <- getLine
		interact $ process $ read n

process :: Int -> String -> String
process n input = unlines [shorten line | line <- take n (lines input)]
		
shorten :: String -> String
shorten s = if length s > 10 then abbrev s else s
				where abbrev s = [head s] ++ (show . flip (-) 2 $ length s) ++ [last s]