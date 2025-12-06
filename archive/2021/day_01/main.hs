main :: IO ()
main = do
    contents <- getContents
    let numbers   = map read (lines contents) :: [Int]
    putStrLn $ "Part 1: " ++ show (part1 numbers)
    putStrLn $ "Part 2: " ++ show (part2 numbers)

part1 :: [Int] -> Int
part1 numbers =
    let pairs     = zip numbers (tail numbers)
        increases = filter (uncurry (<)) pairs
    in length increases

part2 :: [Int] -> Int
part2 numbers =
    let pairs     = zip3 numbers (tail numbers) (tail (tail numbers))
        sums      = map (\(a, b, c) -> a + b + c) pairs
        increases = filter (uncurry (<)) $ zip sums (tail sums)
    in length increases
