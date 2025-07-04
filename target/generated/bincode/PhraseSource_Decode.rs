impl < __Context > :: bincode :: Decode < __Context > for PhraseSource
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            key : :: bincode :: Decode :: decode(decoder) ?, from_idx : ::
            bincode :: Decode :: decode(decoder) ?, to_idx : :: bincode ::
            Decode :: decode(decoder) ?,
        })
    }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for PhraseSource
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            key : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, from_idx : :: bincode :: BorrowDecode
            ::< '_, __Context >:: borrow_decode(decoder) ?, to_idx : ::
            bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?,
        })
    }
}