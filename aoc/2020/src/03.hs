module Something where

import Data.List
import Control.Applicative

type Field = [String]
type HP = (Int, Int)  -- hits and horizontal position

queries = [ (1,1)
          , (3,1)
          , (5,1)
          , (7,1)
          , (1,2)
          ]

hcounters :: [Field -> Int]
hcounters = map gh queries
    where gh :: (Int, Int) -> Field -> Int
          gh (sk,sl) = fst . countHits sk sl

main :: IO ()
main = do
    field <- getLines
    let results = map ($ field) hcounters
    print $ foldl (*) 1 results


getLines :: IO Field
getLines = getContents >>= return . filter ((>0) . length) . lines

countHits :: Int -> Int -> Field -> HP
countHits skip slope = countSlopeHits slope . every skip
    where every i [] = []
          every i (x:xs) = x : (every i $ drop (i-1) xs)

countSlopeHits :: Int -> Field -> HP
countSlopeHits slope = foldl (accHits slope) (0,0)

accHits :: Int -> HP -> String -> HP
accHits slope (h, pos) s = (h + hits s pos, (pos + slope) `mod` length s)

hits :: String -> Int -> Int
hits s i = fromEnum $ hasAt '#' s i

countTrue :: [Bool] -> Int
countTrue = sum . map fromEnum


hasAt :: Eq a => a -> [a] -> Int -> Bool
hasAt _ [] _ = False
hasAt e (x:xs) 0 = x == e
hasAt e (x:xs) i = hasAt e xs (i-1)

