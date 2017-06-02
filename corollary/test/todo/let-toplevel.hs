module Let.Toplevel ()
where

-- Accepting the parse
-- If the current token is (1), it means we've just accepted a partial
-- parse (a %partial parser).  We must ignore the saved token on the top of
-- the stack in this case.
happyAccept (1) tk st sts (_ `HappyStk` ans `HappyStk` _) = happyReturn1 ans
happyAccept j tk st sts (HappyStk ans _) = (happyReturn1 ans)
