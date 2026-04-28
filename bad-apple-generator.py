import cv2
import struct

vidcap = cv2.VideoCapture('bad-apple-scaled.mp4')

success,image = vidcap.read()
count = 0

white = False
rle_value = 0

def is_white(pxl):
    total = int(pxl[0]) + int(pxl[1]) + int(pxl[2])

    if total > 483:
        return True
    
    return False

with open('out.bin', 'wb') as out:
    while success:
        for y in range(160):
            for x in range(240):
                pxl = image[y, x]
                pixel_is_white = is_white(pxl)

                if white == pixel_is_white:
                    rle_value += 1

                    if rle_value == 255:
                        out.write(struct.pack("=B", 255))
                        out.write(struct.pack("=B", 0))
                        rle_value = 0
                else:
                    out.write(struct.pack("=B", rle_value))
                    white = pixel_is_white
                    rle_value = 1

        success, image = vidcap.read()
        print('Read a new frame: ', success)
        count += 1

    out.write(struct.pack("=B", rle_value))
