 > Context: The size of the convolutional filter, also known as the sliding window, is an important parameter in convolutional neural networks(CNNs). The size of the filter determines the receptive filed of the network and influences the level of detail captured in the feature maps.

 Here is some information about the sliding window/convolutional filter:
 * The size of the filter is typicallt specified as a square matrix, such as 3x3 ro 5x5.
 * The third dimension of the filter corresponds to the number of channels in the input image. For example, a 3x3x1 filter is used for grayscale images, while a 3x3x3 filter is used for colored images with three channels(red, green, blue).
 * The filter is slid over the input image or feature map, and at each location, a convolution operation is performed.
 * The stride parameter determines how much the filter moves at each step. By default, the stride is usually set to 1, meaning the filter moves one pixel at a time.
* The padding parameter can also affect the size of the output feature map. Padding adds extra pixels around the input image to preserve its spatial dimensions.

In summary, the size of the convolutional filter is typically specified as a square matrix, and the third dimension corresponds to the number of channels in the input image. The stride and padding parameters can also affect the size of the output feature map.


## The term "Sliding Window"

It is in the context of convolutional neural networks (CNNs) refers to a rectangular region with a defined width and heigh that moves over an image. Here are some possible reasons whty we call it a sliding window:
* The sliding window moves over the input image of feature mao, and at each location, a convolution operation is performed. The window `slide` over the image, hence the name `sliding window`.
* The sliding window is a rectangular region that moves over an image, allowing us localize exactly "where" in an image an objects sits. The window `slides` over the image, hence the name `sliding window`.

In summary, the term `sliding window` is used to describe a rectangular region with a defined width and height that moves over an image ot feature map. The window `slides` over the input data, allowing us to perform convolution operations or object detection at different locations.

### Sliding Window Algorithm

The sliding window algorithm is a method for performing operations on sequences such as arrays and strings. By using this method, time complexity can be reduced from $O(n3)$ to $O(n2)$ or from $O(n2)$ to $O(n)$. As the subarray moves from one end of the array to the other, it looks like a sliding window.

### Some of use cases
* The  problems involves data structures such as arrays, lists and strings. An image is basically a multi-dimensional array.
* We want to find a surange involing the longest, shortest, or goal value in the array of string.
* Conceptually, it revoles around ideas like the longest ot shortest sequnce of somthing that meets a specific requirement.

### How it works

Consider the following array as an example:

$$[a,b,c,d,e,f,g,h]$$

A sliding window would pass over the array as follows.

<figure><img><img src="https://hostux.social/system/media_attachments/files/110/733/220/146/982/235/original/98cf6d4c544ebf4b.webp" alt="" width="1000"><figcaption><p>Source from Microsoft Learning</p></figcaption></figure>


#### The sliding window technique operates according to the fllowing steps:
* Determine the required window size
* Begin with the data structure's first window
* In a loop, slide the window by 1 and continue calculating the result window by window.

{% embed url="https://gist.github.com/Aisuko/9551c74b3fedd83ec6ba1e0c2a022e3c" %}
Sliding window
{% endembed %}

#### How the window slides over the data set

Consider an array $arr =[5,2,-1,0,3]$ and value of k=3 and n=5

Here we have calculated the initial window total from index 0 onwards. The window sum is 6 at this point. Using the currect window as the maximum sum, we may set the maximum sum to 6.

<figure><img><img src="https://hostux.social/system/media_attachments/files/110/733/353/804/688/870/original/e1d34cf4f5c604bd.webp" alt="" width="1000"><figcaption><p>Source from Microsoft Learning</p></figcaption></figure>

After that, we move our window by one unit. As a consequence, it now removes 5 from the window and adds 0 at the end. We deduct 5 from our new window sum before adding 0. Now, the new value of our window sum is 1. Thsi window sum will be compared to the current maximum sum. We won't adjust the maximum sum because it's smaller thant he current maximum.

<figure><img><img src="https://hostux.social/system/media_attachments/files/110/733/358/834/848/727/original/db6fd17aeff8ecf0.webp" alt="" width="1000"><figcaption><p>Source from Microsoft Learning</p></figcaption></figure>

This time we move our window by a unit index to get the new window sum of 2. Once more, we check if the current window sum exceeds the maximum sum reached thus far. The maximum sum is not changed because the current sum is smaller than the maximum.

<figure><img><img src="https://hostux.social/system/media_attachments/files/110/733/364/146/639/431/original/ecf1ea9af8ce552d.webp" alt="" width="1000"><figcaption><p>Source from Microsoft Learning</p></figcaption></figure>

This array's maximum sum, thus, is 6.

## Credit:

[The Sliding Window](https://www.kaggle.com/code/ryanholbrook/the-sliding-window)
[Convolutional Neural Network](https://towardsdatascience.com/applied-deep-learning-part-4-convolutional-neural-networks-584bc134c1e2)
[What is the sliding window algorithm](https://programmathically.com/what-is-the-sliding-window-algorithm/)
