#!/usr/bin/env python3

from PIL import Image, ImageChops
import numpy as np


def print_info(info: dict):
    for k, v in info.items():
        print(f"{k!r}:")
        print(v)
    ...


def main():
    ima = Image.open("./IMG_0514.PNG")
    imb = Image.open("./IMG_0515.PNG")
    # print_info(ima.info)
    # print_info(imb.info)
    print(ima.getexif())
    print(imb.getexif())
    # return
    EXIF = "Raw profile type exif"
    # ba = np.array(list(ima.info[EXIF].encode("utf-8")))
    # bb = np.array(list(imb.info[EXIF].encode("utf-8")))
    # diff = np.abs(ba - bb)
    # for i, v in enumerate(ba):
    #     if v == ord("0"):
    #         ba[i] = ord(" ")
    #     elif v != ord("\n") and v != ord(" "):
    #         ba[i] = ord("#")
    # print(bytes(ba).decode("utf-8"))
    # print(bytes(bb).decode("utf-8"))

    # diff = ImageChops.difference(ima, imb)
    # print(ima == imb, diff.getbbox())
    # print(diff.info)


if __name__ == "__main__":
    main()
