# compile Corrode to Rust

Yeah

http://code.haskell.org/language-c/src/Language/C/Syntax/Ops.hs

```hs
-- | C binary operators (K&R A7.6-15)
--
data CBinaryOp = CMulOp
               | CDivOp
               | CRmdOp                 -- ^ remainder of division
               | CAddOp
               | CSubOp
               | CShlOp                 -- ^ shift left
               | CShrOp                 -- ^ shift right
               | CLeOp                  -- ^ less
               | CGrOp                  -- ^ greater
               | CLeqOp                 -- ^ less or equal
               | CGeqOp                 -- ^ greater or equal
               | CEqOp                  -- ^ equal
               | CNeqOp                 -- ^ not equal
               | CAndOp                 -- ^ bitwise and
               | CXorOp                 -- ^ exclusive bitwise or
               | COrOp                  -- ^ inclusive bitwise or
               | CLndOp                 -- ^ logical and
               | CLorOp                 -- ^ logical or
               deriving (Eq,Ord,Show,Data,Typeable)

isCmpOp :: CBinaryOp -> Bool
isCmpOp op = op `elem` [ CLeqOp, CGeqOp, CLeOp, CGrOp, CEqOp, CNeqOp ]
```
