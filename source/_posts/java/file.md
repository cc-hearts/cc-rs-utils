---
title: file 文件流
categories: Java
---

## 基本文件流的使用

```java
package file;

import org.junit.Test;

import java.io.File;
import java.io.FileReader;
import java.io.IOException;

/**
 * 基本文件流操作
 */
public class BaseFile {
    public static void main(String[] args) {
        BaseFile baseFile = new BaseFile();
        baseFile.readFile();
    }

    /**
     * file 基本使用
     */
    public void baseFile() {
        File file = new File("hello.txt");
        System.out.println(file.getAbsoluteFile());
        // /Users/heart/Desktop/java/stu/hello.txt 相较于当前工程
    }

    /**
     * 文件数据读取
     */
    public void readFile() {
        FileReader fr = null;
        try {
            fr = new FileReader("src/file/base-file.txt");
            while (true) {
                int a = fr.read();
                if (a == -1) break;
                // 表现形式为 uft8 可以使用char转为字符串
                // 换行符的 操作为 10
                System.out.println(a);
            }

        } catch (Error | IOException e) {
            // 文件如果不存在 则会报错 FileNotFoundException
            e.printStackTrace();
        } finally {
            try {
                // 关闭流的读取操作
                // JVM无法对自动对I/O流进行关闭 因此 需要我们自己去关闭 否则会造成内存泄漏
                // 保证流的能够正常的关闭
                if (fr != null) {
                    fr.close();
                }
            } catch (IOException e) {
                e.printStackTrace();
            }
        }
    }


    @Test
    public void test() {
        File file = new File("hello.txt");
        System.out.println(file.getAbsoluteFile());
        // 相较于当前的Module
        // /Users/heart/Desktop/java/stu/hello.txt
    }

}

```
