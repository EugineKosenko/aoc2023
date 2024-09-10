read =: 1!:1 @ <
format =: (LF&= ,;._2 ]) @ read
irows =: i.(@(0&{@$))
icols =: i.(@(1&{@$))
iall =: ,/ @ ({ @ (irows ; icols))
ipos =: > @ (('#'= ,/) # iall)
ecols =: (*./ @ ('#'&~:)) # (i. @ #)
erows =: ecols &.|:
shift_axis =: [ + (999999&* @ (+/ @: >))
shift_row =: (0{[) shift_axis (erows @ ])
shift_col =: (1{[) shift_axis (ecols @ ])
shift_pos =: shift_row , shift_col
shifted =: ipos shift_pos"(1 _) ]
dist =: +/@:(|@-)
dists =: dist"1/~ @ shifted
sum =: (x:@(-:@(+/@,/)))@dists
result =: sum @ format
result =: result f.
