impl < __Context > :: bincode :: Decode < __Context > for LemmaWord
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            lemma_id : :: bincode :: Decode :: decode(decoder) ?,
            word_location : :: bincode :: Decode :: decode(decoder) ?, text :
            :: bincode :: Decode :: decode(decoder) ?,
        })
    }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for LemmaWord
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            lemma_id : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, word_location : :: bincode ::
            BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?, text
            : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?,
        })
    }
}