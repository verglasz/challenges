import System.IO
import Prelude
import Data.Char

readMe :: IO String
readMe = readFile "./aoc02"

dropUntil :: (a -> Bool) -> [a] -> [a]
dropUntil _ [] = []
dropUntil p (x:xs) | p x       = xs
                   | otherwise = dropUntil p xs

getLines :: String -> [String]
getLines [] = []
getLines str = takeWhile (/='\n') str : getLines (dropUntil (=='\n') str)

splitString :: String -> [String]
splitString [] = []
splitString xs = takeWhile isAlphaNum xs : splitString (dropWhile (not . isAlphaNum) $ dropUntil (not . isAlphaNum) xs)

rearrange :: [String] -> (Int,Int,Char,String)
rearrange (x1:x2:x3:x4:_) = (read x1,read x2,head x3,x4)

isValid :: (Int,Int,Char,String) -> Bool
isValid (x1,x2,x3,x4) = ((x4 !! (x1 - 1)) == x3 || (x4 !! (x2 - 1)) == x3) && not ((x4 !! (x1 - 1)) == x3 && (x4 !! (x2 - 1)) == x3)

solve :: String -> Int
solve = length . filter isValid . map (rearrange . splitString) . getLines

main = readMe >>= (print . solve)
