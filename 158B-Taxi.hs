
main = do
        _ <- getLine
        l <- getLine
        putStrLn $ show $ solve $ map read $ words l
        
-- solve :: [Int] -> Int
-- solve gs = gof 4 gs + gof 3 gs + gof_2 + gof_1
            -- where
                -- gof_2 = let r = gof 2 gs in r `quot` 2 + r `rem` 2
                -- gof_1
                    -- | gof_3_1 < 0   = 0
                    -- | gof_3_2_1 < 0 = 0
                    -- | otherwise     = gof_3_2_1 `quot` 4 + (if gof_3_2_1 `rem` 4 > 0 then 1 else 0)
                      -- where gof_3_1   = gof 1 gs - gof 3 gs
                            -- gof_3_2_1 = gof_3_1 - (if odd gof 2 gs then 2 else 0)
                            
count :: Int -> [Int] -> Int
count x xs = length $ filter (==x) xs

solve :: [Int] -> Int
solve xs = n_4 + n_3 + n_2 + n_1
            where n_4 = count 4 xs
                  n_3 = count 3 xs
                  n_2 = ceiling $ (fromIntegral $ count 2 xs) / 2.0 -- groups of 2 people fit into one more than the # of groupd div 2
                  leftover = (count 1 xs) - n_3 - (n_2 `mod` 2 * 2) -- get the number of single person groups left over after filling up the 3-cars and the odd 2 person car
                  n_1 =  if leftover > 0 then ceiling $ (fromIntegral leftover) / 4.0 else 0
