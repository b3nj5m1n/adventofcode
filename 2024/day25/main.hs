import Data.ByteString (split)
import Data.Char (isSpace)
import Data.Text hiding (filter, head, init, length, map, tail, zip)
import System.Environment (getArgs)
import System.Exit

type Results = (Maybe Int, Maybe Int)

main = getArgs >>= parseArgs >>= readFile >>= solve >>= printResult

printResult :: Results -> IO ()
printResult (Just part_1, Just part_2) = putStrLn ("Part 1: " ++ show part_1 ++ "\nPart 2: " ++ show part_2)
printResult (Just part_1, Nothing) = putStrLn ("Part 1: " ++ show part_1)
printResult (Nothing, Just part_2) = putStrLn ("Part 2: " ++ show part_2)

parseArgs :: [String] -> IO String
parseArgs [filename] = return filename
parseArgs _ = putStrLn "Provide filename as argument" >> exitWith (ExitFailure 1)

isLock :: [Text] -> Bool
isLock = (== pack "#####") . head

solve :: String -> IO Results
solve file_content = do
  let parsed =
        map (splitOn (pack "\n"))
          . splitOn (pack "\n\n")
          $ dropWhileEnd isSpace (pack file_content)
  let locks =
        (map (map (count (pack "#")) . transpose . init . tail) . filter isLock) parsed
  let keys =
        (map (map (count (pack "#")) . transpose . init . tail) . filter (not . isLock)) parsed
  let valid_combinations = filter (\xs -> length xs == 0) [filter (\x -> x > 5) (map (\(x, y) -> x + y) (zip x y)) | x <- locks, y <- keys]
  return (Just (length valid_combinations), Nothing)
