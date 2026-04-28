import cv2
import struct

vidcap = cv2.VideoCapture('bad-apple-scaled.mp4')

success,image = vidcap.read()
count = 0
pixel_count = 0
out_val = 0

out = open('out.bin', 'wb')

while success:
  for x in range(160):
    for y in range(240):
        pxl = image[x,y]

        total = int(pxl[0]) + int(pxl[1]) + int(pxl[2])

        if total > 483:
            out_val = out_val | 1

        pixel_count +=1
        
        if pixel_count == 8:
            out.write(struct.pack("=B", out_val))
            out_val = 0
            pixel_count = 0
        else:
            out_val = out_val << 1;


  success,image = vidcap.read()
  print('Read a new frame: ', success)
  count += 1

  if count == 1500:
    exit(1)