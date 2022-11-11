---
title: dp案例
categories: Algorithm
---



# 求两个字符串的最大子串

```java
// dp转移方程 求两个字符串的最大子串
    public int dp(String s1, String s2) {
        int[][] dp = new int[s1.length()][s2.length()]; // 默认会都为0
        int count = 0;
        // 动态转移方程 dp[i] != dp[j] => 0
        // dp[i] == dp[j] dp[i][j] == dp[i - 1][j - 1] + 1; // 减少最后寻找最长字串的查找方式
        //  上述需要注意 i - 1  j - 1的边界问题
        // i == 0 || j == 0 时  此时的dp[i][j] 应该直接是 1 i-1数据下标则越界了
        for (int i = 0; i < s1.length(); i++) {
            for (int j = 0; j < s2.length(); j++) {
                if (s1.charAt(i) == s2.charAt(j)) {
                    if (i == 0 || j == 0) {
                        dp[i][j] = 1;
                    } else {
                        dp[i][j] = dp[i - 1][j - 1] + 1;
                        count = Math.max(dp[i][j], count);
                    }
                }
            }
        }
        return count;
    }
```
