impl < __Context > :: bincode :: Decode < __Context > for TopicEntry
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            topic_id : :: bincode :: Decode :: decode(decoder) ?, name : ::
            bincode :: Decode :: decode(decoder) ?, arabic_name : :: bincode
            :: Decode :: decode(decoder) ?, parent_id : :: bincode :: Decode
            :: decode(decoder) ?, description : :: bincode :: Decode ::
            decode(decoder) ?, ayahs : :: bincode :: Decode :: decode(decoder)
            ?, keywords : :: bincode :: Decode :: decode(decoder) ?,
            total_ayahs : :: bincode :: Decode :: decode(decoder) ?,
        })
    }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for TopicEntry
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            topic_id : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, name : :: bincode :: BorrowDecode ::<
            '_, __Context >:: borrow_decode(decoder) ?, arabic_name : ::
            bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, parent_id : :: bincode :: BorrowDecode
            ::< '_, __Context >:: borrow_decode(decoder) ?, description : ::
            bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, ayahs : :: bincode :: BorrowDecode ::<
            '_, __Context >:: borrow_decode(decoder) ?, keywords : :: bincode
            :: BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?,
            total_ayahs : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?,
        })
    }
}