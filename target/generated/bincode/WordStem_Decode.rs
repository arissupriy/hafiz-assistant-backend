impl < __Context > :: bincode :: Decode < __Context > for WordStem
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            stem_id : :: bincode :: Decode :: decode(decoder) ?, word_location
            : :: bincode :: Decode :: decode(decoder) ?, text : :: bincode ::
            Decode :: decode(decoder) ?,
        })
    }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for WordStem
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            stem_id : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, word_location : :: bincode ::
            BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?, text
            : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?,
        })
    }
}