
main = do
        _ <- getLine
        l <- getLine
        print $ solve $ map read $ words l
                            
count :: Int -> [Int] -> Int
count x xs = length $ filter (==x) xs

solve :: [Int] -> Int
solve xs = n_4 + n_3 + n_2 + n_1
            where n_4 = count 4 xs
                  n_3 = count 3 xs
                  n_2 = ceiling $ (fromIntegral $ count 2 xs) / 2.0 -- groups of 2 people fit into one more than the # of groupd div 2
                  leftover = (count 1 xs) - n_3 - ((count 2 xs) `mod` 2 * 2) -- get the number of single person groups left over after filling up the 3-cars and the odd 2 person car
                  n_1 =  if leftover > 0 then ceiling $ (fromIntegral leftover) / 4.0 else 0
