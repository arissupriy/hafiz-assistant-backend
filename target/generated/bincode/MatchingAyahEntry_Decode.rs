impl < __Context > :: bincode :: Decode < __Context > for MatchingAyahEntry
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            matched_ayah_key : :: bincode :: Decode :: decode(decoder) ?,
            matched_words_count : :: bincode :: Decode :: decode(decoder) ?,
            coverage : :: bincode :: Decode :: decode(decoder) ?, score : ::
            bincode :: Decode :: decode(decoder) ?, match_words : :: bincode
            :: Decode :: decode(decoder) ?,
        })
    }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for MatchingAyahEntry
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            matched_ayah_key : :: bincode :: BorrowDecode ::< '_, __Context
            >:: borrow_decode(decoder) ?, matched_words_count : :: bincode ::
            BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?,
            coverage : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, score : :: bincode :: BorrowDecode ::<
            '_, __Context >:: borrow_decode(decoder) ?, match_words : ::
            bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?,
        })
    }
}