module Main where

import Solutions (solutions)

main :: IO ()
main = do
    args <- getArgs
    case args of
        [] -> return
        x:_ ->  runSolution $ read x


runSolution :: Int -> IO ()
runSolution = (solutions !!)

