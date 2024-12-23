module Solutions where

import Text.Read
import Data.Maybe
import Data.Tuple.Sequence
import Data.Either
import Control.Monad
import System.IO

type PassData = (Int, Int, Char, String)

solve :: IO ()
solve = do
    input <- getContents
    let parsed = map parse $ lines input
    putStr "Valid according to range-rule: "
    print $ countValid isValidRange parsed
    putStr "Valid according to point-rule: "
    print $ countValid isValidPoint parsed


countValid :: (a -> Bool) -> [a] -> Int
countValid f = foldl accf 0
    where accf = flip $ (+) . fromEnum . f

getPasswords :: IO [PassData]
getPasswords = isEOF >>= gp
    where gp True  = return []
          gp False = liftM2 (:) (fmap parse getLine) getPasswords

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
              unlist xs = error $ show xs ++ " is not a one-element list"
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

