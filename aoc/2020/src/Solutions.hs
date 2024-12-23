module Solutions where

import Data.List
import Control.Applicative
import Text.Read
import Data.Maybe
import Data.Tuple.Sequence
import Data.Either
import Control.Monad
import System.IO

solutions :: [IO ()]
solutions =
    [ putStrLn "run the python solver instead"
    , solve02
    , solve03
    ]

-- EXERCISE 2
type PassData = (Int, Int, Char, String)

solve02 :: IO ()
solve02 = do
    input <- getContents
    let parsed = map parse $ lines input
    putStr "Valid according to range-rule: "
    print $ countValid isValidRange parsed
    putStr "Valid according to point-rule: "
    print $ countValid isValidPoint parsed


countValid :: (a -> Bool) -> [a] -> Int
countValid f = foldl accf 0
    where accf = flip $ (+) . fromEnum . f

-- getPasswords :: IO [String]
-- getPasswords = isEOF >>= gp
--     where gp True  = return []
--           gp False = liftM2 (:) (fmap parse getLine) getPasswords

hasAt :: Eq a => Int -> a -> [a] -> Bool
hasAt _ _ [] = False
hasAt 0 el (x:xs) = el == x
hasAt i el (x:xs) = hasAt (i-1) el xs

isValidPoint :: PassData -> Bool
isValidPoint (a, b, ch, pw) = at a `xor` at b
            where at i = hasAt (i-1) ch pw
                  xor True = not
                  xor False = id

isValidRange :: PassData -> Bool
isValidRange (min,max,ch,pw) = let n = count ch pw in min <= n && n <= max

count :: Eq a => a -> [a] -> Int
count el = sum . map (fromEnum . (el ==))

parse :: String -> PassData
parse s = let (a,b,c,d) = split s
              unlist [el] = el
              unlist xs   = error $ show xs ++ " is not a one-element list"
            in (read a, read b, unlist c, d)

split s = let (a, rest) = splitUntil "-" s
              (b, rest') = splitUntil " " rest
              (c, d) = splitUntil ": " rest'
        in (a,b,c,d)

splitUntil :: Eq a => [a] -> [a] -> ([a], [a])
splitUntil [] ls = ([],ls)
splitUntil _ [] = ([], [])
splitUntil ps (x:xs) = case withoutPrefix ps $ x:xs of
                        Right rest -> ([], rest)
                        Left no -> let (pre, post) = splitUntil ps xs in (x:pre, post)


withoutPrefix :: Eq a => [a] -> [a] -> Either [a] [a]
withoutPrefix [] ls = Right ls
withoutPrefix _ [] = Left []
withoutPrefix (p:ps) (x:xs) | p /= x    = Left $ x:xs
                            | otherwise = case withoutPrefix ps xs of
                                            Left no -> Left $ x:no
                                            r -> r

-- EXERCISE 3


type Field = [String]
type HP = (Int, Int)  -- hits and horizontal position

queries = [ (1,1)
          , (3,1)
          , (5,1)
          , (7,1)
          , (1,2)
          ]

hcounters :: [Field -> Int]
hcounters = map gethits queries
    where gethits :: (Int, Int) -> Field -> Int
          gethits (sk,sl) = countHits sk sl

solve03 :: IO ()
solve03 = do
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

