module Test()
where

interpretStatement :: Bool
interpretStatement stmt@(CSwitch expr body node) next = do
    (_, SwitchCases cases) <- getSwitchCases expr' $ setBreak after $
        interpretStatement body (return ([], Branch after))