impl < __Context > :: bincode :: Decode < __Context > for Phrase
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            surahs : :: bincode :: Decode :: decode(decoder) ?, ayahs : ::
            bincode :: Decode :: decode(decoder) ?, count : :: bincode ::
            Decode :: decode(decoder) ?, source : :: bincode :: Decode ::
            decode(decoder) ?,
        })
    }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for Phrase
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            surahs : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, ayahs : :: bincode :: BorrowDecode ::<
            '_, __Context >:: borrow_decode(decoder) ?, count : :: bincode ::
            BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?,
            source : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?,
        })
    }
}