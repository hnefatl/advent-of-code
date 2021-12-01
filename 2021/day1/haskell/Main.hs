module Main where

import Data.List (tails, transpose)

part1 :: [Int] -> Int
part1 depths = length . filter id $ zipWith (<) depths (drop 1 depths)

part2 :: [Int] -> Int
part2 = part1 . map sum . transpose . take 3 . tails

pureMain :: String -> String
pureMain s = unlines $ map show [part1 depths, part2 depths]
  where
    depths = map read $ lines s

main :: IO ()
main = interact pureMain