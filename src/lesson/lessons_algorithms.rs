use super::Lesson;

pub fn lessons() -> Vec<Lesson> {
    vec![
        // ── 1. Two Pointers ────────────────────────────────────────────────
        Lesson {
            id: "two-pointers",
            category: "Interview Algorithms",
            title: "Two Pointers",
            description: r#"<p>The <b>two-pointer</b> pattern uses two indices that move toward (or away from) each other through a sorted array or string. Many O(n²) brute-force searches become O(n) with two pointers.</p>
<h3>Classic applications</h3>
<ul>
  <li>Remove duplicates from a sorted Vec in-place — <b>O(n) time, O(1) space</b></li>
  <li>Reverse an array or string in-place — <b>O(n) time, O(1) space</b></li>
  <li>Two-sum on a sorted array — <b>O(n) time, O(1) space</b></li>
  <li>Container with most water — <b>O(n) time, O(1) space</b></li>
</ul>
<h3>Rust-specific notes</h3>
<ul>
  <li><code>slice.swap(i, j)</code> swaps two elements in-place without a temp variable.</li>
  <li>In-place mutations require <code>&amp;mut [T]</code> or <code>&amp;mut Vec&lt;T&gt;</code>.</li>
  <li>Use <code>.saturating_sub(1)</code> on <code>usize</code> for the right pointer to avoid underflow on empty slices.</li>
  <li>Prefer <code>&amp;mut [T]</code> over <code>&amp;mut Vec&lt;T&gt;</code> in function signatures — it's more general.</li>
</ul>"#,
            code: r#"// Remove duplicates from a sorted Vec in-place.
// Returns the number of unique elements; the first k elements hold the result.
// Time: O(n)  Space: O(1)
fn remove_duplicates(v: &mut Vec<i32>) -> usize {
    if v.is_empty() {
        return 0;
    }
    let mut write = 1usize; // slow pointer
    for read in 1..v.len() { // fast pointer
        if v[read] != v[write - 1] {
            v[write] = v[read];
            write += 1;
        }
    }
    v.truncate(write);
    write
}

// Reverse a byte slice in-place using slice.swap(i, j).
// Time: O(n)  Space: O(1)
fn reverse_in_place(s: &mut [u8]) {
    let mut l = 0;
    let mut r = s.len().saturating_sub(1);
    while l < r {
        s.swap(l, r); // swap without a temp variable
        l += 1;
        r -= 1;
    }
}

// Two-sum on a sorted slice: find indices where a[i] + a[j] == target.
// Time: O(n)  Space: O(1)
fn two_sum_sorted(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut l = 0;
    let mut r = nums.len().saturating_sub(1);
    while l < r {
        match (nums[l] + nums[r]).cmp(&target) {
            std::cmp::Ordering::Equal   => return Some((l, r)),
            std::cmp::Ordering::Less    => l += 1,
            std::cmp::Ordering::Greater => r -= 1,
        }
    }
    None
}

fn main() {
    let mut v = vec![1, 1, 2, 3, 3, 4, 5, 5];
    let k = remove_duplicates(&mut v);
    println!("Remove duplicates: k={}, result={:?}", k, v);

    let mut s = b"hello world".to_vec();
    reverse_in_place(&mut s);
    println!("Reversed bytes  : {}", std::str::from_utf8(&s).unwrap());

    let sorted = vec![1, 3, 4, 6, 8, 11];
    println!("Two-sum target=9: {:?}", two_sum_sorted(&sorted, 9));  // (1,4) => 3+6=9? No, 3+6=9 nope: check
    println!("Two-sum target=7: {:?}", two_sum_sorted(&sorted, 7));  // (0,3) => 1+6=7
}"#,
            notes: vec![
                "Two pointers: O(n) replacement for O(n²) pair search when the array is sorted or the invariant is monotone.",
                "slice.swap(i, j) is idiomatic Rust for in-place swaps — no temp variable needed.",
                "Use usize arithmetic carefully: usize underflow panics in debug builds. saturating_sub(1) is the safe idiom.",
                "Prefer &mut [T] over &mut Vec<T> in function params — slices are more general and auto-deref from Vec.",
                "Interview tip: remove-duplicates in-place is a classic warm-up; get the write-pointer logic right.",
            ],
        },

        // ── 2. Sliding Window ──────────────────────────────────────────────
        Lesson {
            id: "sliding-window",
            category: "Interview Algorithms",
            title: "Sliding Window",
            description: r#"<p>The <b>sliding window</b> pattern maintains a range <code>[left, right)</code> over a sequence and expands or contracts it based on a constraint. It reduces O(n²) or O(n·k) algorithms to <b>O(n)</b>.</p>
<h3>Two variants</h3>
<ul>
  <li><b>Fixed-size window</b>: move window of size k one step at a time — add the new element, subtract the evicted one. Classic: maximum sum subarray of size k.</li>
  <li><b>Variable-size window</b>: expand right until constraint is violated, then shrink left. Classic: longest substring without repeating characters.</li>
</ul>
<h3>Rust notes</h3>
<ul>
  <li><code>HashMap&lt;char, usize&gt;</code> tracks the last-seen index of each character.</li>
  <li>Characters in Rust are Unicode scalars; collect to <code>Vec&lt;char&gt;</code> first if you need O(1) index access — <code>&amp;str</code> indexing is by bytes.</li>
  <li><code>usize::max(a, b)</code> or <code>a.max(b)</code> for updating the running maximum.</li>
</ul>"#,
            code: r#"use std::collections::HashMap;

// Longest substring without repeating characters.
// Time: O(n)  Space: O(min(n, alphabet))
fn longest_no_repeat(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect(); // O(1) index access
    let mut last_seen: HashMap<char, usize> = HashMap::new();
    let mut max_len = 0usize;
    let mut left = 0usize;

    for (right, &ch) in chars.iter().enumerate() {
        if let Some(&prev_idx) = last_seen.get(&ch) {
            if prev_idx >= left {
                left = prev_idx + 1; // shrink window past the duplicate
            }
        }
        last_seen.insert(ch, right);
        max_len = max_len.max(right - left + 1);
    }
    max_len
}

// Maximum sum subarray of fixed size k.
// Time: O(n)  Space: O(1)
fn max_sum_subarray(nums: &[i64], k: usize) -> i64 {
    assert!(nums.len() >= k, "array must have at least k elements");
    let mut window: i64 = nums[..k].iter().sum();
    let mut max = window;
    for i in k..nums.len() {
        window += nums[i] - nums[i - k]; // add new, subtract evicted
        max = max.max(window);
    }
    max
}

fn main() {
    let cases = ["abcabcbb", "bbbbb", "pwwkew", ""];
    for s in &cases {
        println!("longest_no_repeat({:?}) = {}", s, longest_no_repeat(s));
    }
    // expected: 3, 1, 3, 0

    println!();
    let nums = vec![2i64, 3, 4, 1, 5, 9, 2];
    for k in [2, 3, 4] {
        println!("max_sum_subarray k={}: {}", k, max_sum_subarray(&nums, k));
    }
    // expected: 14 (9+5), 15 (5+9+1? no: 1+5+9=15), 17 (1+5+9+2=17)
}"#,
            notes: vec![
                "Variable window: expand right freely, shrink left only when the constraint is violated — amortized O(n).",
                "Fixed window: add nums[i], subtract nums[i-k] — one pass, constant space.",
                "&str indexing is by byte offset, not char. Collect to Vec<char> for O(1) Unicode char access.",
                "HashMap char→last_index lets you jump the left pointer past a duplicate in O(1).",
                "Interview tip: identify whether the window is fixed or variable first — it determines the loop structure.",
            ],
        },

        // ── 3. Binary Search ───────────────────────────────────────────────
        Lesson {
            id: "binary-search",
            category: "Interview Algorithms",
            title: "Binary Search and Variants",
            description: r#"<p>Binary search finds a target in a <b>sorted</b> slice in <b>O(log n)</b>. Beyond simple lookup, it solves "first position", "last position", "insertion point", and "minimum satisfying value" problems.</p>
<h3>Rust stdlib</h3>
<p><code>slice::binary_search(&amp;target)</code> returns <code>Ok(idx)</code> if found, or <code>Err(insertion_point)</code> if not. The insertion point is where the value <em>would</em> be inserted to keep the slice sorted.</p>
<h3>Overflow</h3>
<p>The classic mid-point bug: <code>(lo + hi) / 2</code> overflows when both are large. In Rust, debug builds panic on integer overflow — which is actually helpful for finding this bug. Always use <code>lo + (hi - lo) / 2</code>.</p>
<h3>lower_bound / upper_bound</h3>
<p><code>lower_bound(slice, x)</code> → index of first element <code>&gt;= x</code>. <code>upper_bound(slice, x)</code> → index of first element <code>&gt; x</code>. Together they bracket the range of equal elements in O(log n).</p>"#,
            code: r#"// First index where slice[i] >= target. Returns slice.len() if all elements < target.
// Time: O(log n)  Space: O(1)
fn lower_bound(slice: &[i32], target: i32) -> usize {
    let mut lo = 0usize;
    let mut hi = slice.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2; // avoids overflow vs (lo + hi) / 2
        if slice[mid] < target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

// First index where slice[i] > target.
fn upper_bound(slice: &[i32], target: i32) -> usize {
    let mut lo = 0usize;
    let mut hi = slice.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if slice[mid] <= target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

fn main() {
    let v = vec![1i32, 3, 5, 5, 7, 9, 11];

    // --- stdlib binary_search ---
    match v.binary_search(&5) {
        Ok(i)  => println!("Found 5 at index {} (one of possibly many)", i),
        Err(i) => println!("5 not found; insertion point = {}", i),
    }
    match v.binary_search(&6) {
        Ok(i)  => println!("Found 6 at index {}", i),
        Err(i) => println!("6 not found; insertion point = {}", i),
    }

    // --- lower_bound and upper_bound ---
    println!();
    for target in [1, 5, 6, 11, 12, 0] {
        let lb = lower_bound(&v, target);
        let ub = upper_bound(&v, target);
        println!(
            "target={:2}  lower_bound={} upper_bound={} (count={})",
            target, lb, ub, ub - lb
        );
    }
    // target=5: lower=2, upper=4, count=2 (two 5s in the slice)

    // --- Binary search on a predicate (find first true) ---
    // "Find the minimum x in 0..n such that x*x >= 50"
    let (mut lo, mut hi) = (0u64, 100u64);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if mid * mid >= 50 { hi = mid; } else { lo = mid + 1; }
    }
    println!("\nSmallest x where x*x >= 50: {} ({}*{}={})", lo, lo, lo, lo*lo);
}"#,
            notes: vec![
                "slice::binary_search returns Ok(idx) | Err(insertion_point) — insertion_point is super useful for inserts.",
                "Always compute mid = lo + (hi - lo) / 2 to avoid overflow. Rust debug mode panics on overflow — helpful!",
                "lower_bound: first index >= target. upper_bound: first index > target. Together they bound the equal range.",
                "Binary search on a predicate: lo..hi where predicate is false,false,…,true,true. Find the first true.",
                "Interview tip: know all three — simple lookup, lower_bound, and predicate-bisect — they cover 90% of cases.",
            ],
        },

        // ── 4. Backtracking ────────────────────────────────────────────────
        Lesson {
            id: "backtracking",
            category: "Interview Algorithms",
            title: "Backtracking",
            description: r#"<p><b>Backtracking</b> explores all candidates recursively and abandons a candidate ("backtracks") as soon as it determines the candidate cannot lead to a valid solution. It's DFS on the decision tree.</p>
<h3>Template</h3>
<ol>
  <li>Record the current partial solution.</li>
  <li>If complete, push a clone to results.</li>
  <li>For each next choice: push, recurse, pop.</li>
</ol>
<h3>Complexity</h3>
<ul>
  <li>Subsets of n elements: <b>O(2ⁿ)</b> subsets, each requiring O(n) to clone → O(n · 2ⁿ)</li>
  <li>Permutations of n elements: <b>O(n · n!)</b></li>
  <li>Pruning (constraint checks before recursing) dramatically reduces constants.</li>
</ul>
<h3>Rust notes</h3>
<ul>
  <li>Backtracking is one of the cases where recursion is genuinely natural — embrace it.</li>
  <li>Pass <code>&amp;mut Vec&lt;T&gt;</code> as the scratch buffer; call <code>.push()</code> before the recursive call and <code>.pop()</code> after.</li>
  <li>Clone the scratch buffer into results only when a complete solution is found — avoids unnecessary allocations.</li>
</ul>"#,
            code: r#"// Generate all subsets (power set) of a slice.
// Time: O(n · 2ⁿ)  Space: O(n) stack depth + O(n · 2ⁿ) output
fn subsets(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut results = Vec::new();
    let mut current = Vec::new();
    backtrack_subsets(nums, 0, &mut current, &mut results);
    results
}

fn backtrack_subsets(
    nums: &[i32],
    start: usize,
    current: &mut Vec<i32>,
    results: &mut Vec<Vec<i32>>,
) {
    results.push(current.clone()); // every prefix is a valid subset
    for i in start..nums.len() {
        current.push(nums[i]); // choose
        backtrack_subsets(nums, i + 1, current, results); // explore
        current.pop(); // un-choose (backtrack)
    }
}

// Generate all permutations of a slice.
// Time: O(n · n!)  Space: O(n) stack depth
fn permutations(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut results = Vec::new();
    let mut current = Vec::new();
    let mut used = vec![false; nums.len()];
    backtrack_perms(nums, &mut current, &mut used, &mut results);
    results
}

fn backtrack_perms(
    nums: &[i32],
    current: &mut Vec<i32>,
    used: &mut Vec<bool>,
    results: &mut Vec<Vec<i32>>,
) {
    if current.len() == nums.len() {
        results.push(current.clone());
        return;
    }
    for i in 0..nums.len() {
        if used[i] { continue; }
        used[i] = true;
        current.push(nums[i]);
        backtrack_perms(nums, current, used, results);
        current.pop();
        used[i] = false;
    }
}

fn main() {
    let nums = [1, 2, 3];

    let sets = subsets(&nums);
    println!("Subsets of {:?} ({} total, expected {}):", nums, sets.len(), 1 << nums.len());
    for s in &sets {
        print!("  {:?}", s);
    }
    println!();

    let perms = permutations(&nums);
    println!("\nPermutations of {:?} ({} total, expected {}):", nums, perms.len(), 6);
    for p in &perms {
        print!("  {:?}", p);
    }
    println!();
}"#,
            notes: vec![
                "Pattern: push, recurse, pop — the scratch Vec is the shared mutable state threaded through the recursion.",
                "Clone into results only on complete solutions (subsets: every prefix; permutations: when current.len() == n).",
                "Pruning: add an early-return guard before the recursive call to cut branches — key for constraint-satisfaction problems.",
                "Rust stack depth defaults to ~8 MB. For very large n, consider iterative approaches or heap-allocated continuations.",
                "Interview tip: always state the time complexity in terms of the output size (O(n · 2ⁿ) for subsets).",
            ],
        },

        // ── 5. Dynamic Programming ─────────────────────────────────────────
        Lesson {
            id: "dynamic-programming",
            category: "Interview Algorithms",
            title: "Dynamic Programming",
            description: r#"<p><b>Dynamic programming</b> (DP) solves a problem by breaking it into overlapping subproblems and storing results to avoid recomputation. Two strategies:</p>
<ul>
  <li><b>Top-down (memoization)</b>: recursive, cache results in a HashMap or Vec. Natural to write, may have function-call overhead.</li>
  <li><b>Bottom-up (tabulation)</b>: iterative, fill a table from base cases. Usually faster (no recursion overhead, better cache locality).</li>
</ul>
<h3>Space optimisation</h3>
<p>Many DP problems only look back one or two rows/cells. You can reduce O(n) table space to O(1) by keeping only the relevant previous values.</p>
<h3>Rust notes</h3>
<ul>
  <li>Use <code>usize</code> for loop indices but <code>u64</code> for the values when n &gt; ~20 (Fibonacci grows exponentially).</li>
  <li><code>Vec::with_capacity(n + 1)</code> for the DP table avoids reallocations.</li>
  <li>Memoization with <code>Vec&lt;Option&lt;u64&gt;&gt;</code> is cleaner than a HashMap when indices are dense integers.</li>
</ul>"#,
            code: r#"// --- Top-down: climbing stairs with memoization ---
// Ways to reach step n taking 1 or 2 steps at a time.
// Time: O(n)  Space: O(n) (memo table + call stack)
fn climb_memo(n: usize, memo: &mut Vec<Option<u64>>) -> u64 {
    if n <= 1 { return 1; }
    if let Some(v) = memo[n] { return v; }
    let result = climb_memo(n - 1, memo) + climb_memo(n - 2, memo);
    memo[n] = Some(result);
    result
}

// --- Bottom-up: tabulation ---
// Time: O(n)  Space: O(n)
fn climb_tabulation(n: usize) -> u64 {
    if n <= 1 { return 1; }
    let mut dp = vec![0u64; n + 1];
    dp[0] = 1;
    dp[1] = 1;
    for i in 2..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    dp[n]
}

// --- O(1) space: only keep the previous two values ---
// Time: O(n)  Space: O(1)
fn climb_optimised(n: usize) -> u64 {
    if n <= 1 { return 1; }
    let (mut a, mut b) = (1u64, 1u64);
    for _ in 2..=n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}

// --- Coin change (classic DP) ---
// Minimum coins to make amount. Return None if impossible.
// Time: O(amount * coins.len())  Space: O(amount)
fn coin_change(coins: &[u64], amount: u64) -> Option<u64> {
    let amt = amount as usize;
    let mut dp = vec![u64::MAX; amt + 1];
    dp[0] = 0;
    for i in 1..=amt {
        for &c in coins {
            if c as usize <= i && dp[i - c as usize] != u64::MAX {
                dp[i] = dp[i].min(dp[i - c as usize] + 1);
            }
        }
    }
    if dp[amt] == u64::MAX { None } else { Some(dp[amt]) }
}

fn main() {
    println!("Climbing stairs (ways to reach step n):");
    let mut memo = vec![None; 50];
    for n in 0..=10 {
        let m = climb_memo(n, &mut memo);
        let t = climb_tabulation(n);
        let o = climb_optimised(n);
        println!("  n={:2}: memo={} tabulation={} optimised={}", n, m, t, o);
    }

    println!("\nCoin change:");
    let coins = [1u64, 5, 10, 25];
    for amount in [0, 11, 30, 41, 3] {
        println!("  amount={:3}: {:?} coins", amount, coin_change(&coins, amount));
    }
}"#,
            notes: vec![
                "Top-down (memo): natural structure, easy to reason about subproblems. Has recursion + hash overhead.",
                "Bottom-up (tabulation): typically faster — fills a Vec in order, better cache locality, no call stack.",
                "Space optimise by rolling the table: if dp[i] only depends on dp[i-1] and dp[i-2], keep just two variables.",
                "Use u64 for values when n > ~20. Fibonacci(93) overflows u64; climbing stairs stays smaller.",
                "Interview tip: always state all three complexities (time / space before and after optimisation).",
            ],
        },

        // ── 6. Graph Traversal ─────────────────────────────────────────────
        Lesson {
            id: "graph-traversal",
            category: "Interview Algorithms",
            title: "Graph Traversal (BFS/DFS)",
            description: r#"<p>Graph traversal visits every reachable node exactly once. The two canonical strategies differ only in the data structure used for the frontier:</p>
<ul>
  <li><b>BFS</b> (breadth-first): <code>VecDeque</code> as a queue → visits nodes layer by layer → finds the <b>shortest path</b> in an unweighted graph.</li>
  <li><b>DFS</b> (depth-first): <code>Vec</code> as a stack (or recursion) → explores as deep as possible first → useful for cycle detection, topological sort, connected components.</li>
</ul>
<h3>Representation</h3>
<p><code>Vec&lt;Vec&lt;usize&gt;&gt;</code> (adjacency list) is idiomatic Rust for sparse graphs. Index <code>i</code> holds the neighbours of node <code>i</code>. Dense graphs can use <code>Vec&lt;Vec&lt;bool&gt;&gt;</code> (adjacency matrix).</p>
<h3>Complexity</h3>
<p>Both BFS and DFS are <b>O(V + E)</b> where V = vertices and E = edges. The visited array is O(V) space.</p>
<h3>Rust notes</h3>
<ul>
  <li><code>std::collections::VecDeque</code> for BFS — O(1) push_back and pop_front.</li>
  <li>Iterative DFS with a <code>Vec</code> stack avoids recursion depth limits for large graphs.</li>
  <li>A <code>Vec&lt;bool&gt;</code> visited set is O(1) lookup when nodes are dense integers 0..n.</li>
</ul>"#,
            code: r#"use std::collections::VecDeque;

type Graph = Vec<Vec<usize>>;

// BFS: returns nodes in visit order and the shortest distances from `start`.
// Time: O(V + E)  Space: O(V)
fn bfs(graph: &Graph, start: usize) -> (Vec<usize>, Vec<Option<usize>>) {
    let n = graph.len();
    let mut visited = vec![false; n];
    let mut dist: Vec<Option<usize>> = vec![None; n];
    let mut order = Vec::new();
    let mut queue = VecDeque::new();

    visited[start] = true;
    dist[start] = Some(0);
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        order.push(node);
        let d = dist[node].unwrap();
        for &nb in &graph[node] {
            if !visited[nb] {
                visited[nb] = true;
                dist[nb] = Some(d + 1);
                queue.push_back(nb);
            }
        }
    }
    (order, dist)
}

// Iterative DFS using a Vec stack — avoids recursion depth limits.
// Time: O(V + E)  Space: O(V)
fn dfs(graph: &Graph, start: usize) -> Vec<usize> {
    let mut visited = vec![false; graph.len()];
    let mut order = Vec::new();
    let mut stack = vec![start];

    while let Some(node) = stack.pop() {
        if visited[node] { continue; }
        visited[node] = true;
        order.push(node);
        // Push neighbours in reverse so we visit them in forward order
        for &nb in graph[node].iter().rev() {
            if !visited[nb] {
                stack.push(nb);
            }
        }
    }
    order
}

fn main() {
    // Graph:  0 ── 1 ── 3
    //         |    |
    //         2    4
    let graph: Graph = vec![
        vec![1, 2], // node 0
        vec![0, 3, 4], // node 1
        vec![0],    // node 2
        vec![1],    // node 3
        vec![1],    // node 4
    ];

    let (bfs_order, dist) = bfs(&graph, 0);
    println!("BFS order from 0: {:?}", bfs_order);
    println!("Distances from 0: {:?}", dist);

    let dfs_order = dfs(&graph, 0);
    println!("DFS order from 0: {:?}", dfs_order);

    // Shortest path from 0 to 3
    let target = 3;
    match dist[target] {
        Some(d) => println!("\nShortest path 0→{}: {} hops", target, d),
        None    => println!("Node {} not reachable", target),
    }
}"#,
            notes: vec![
                "BFS = queue (VecDeque). DFS = stack (Vec) or recursion. The only structural difference.",
                "BFS finds shortest path in unweighted graphs because it explores layer by layer.",
                "Vec<bool> visited prevents revisiting — essential for graphs with cycles.",
                "VecDeque::push_back / pop_front is O(1) amortised; much better than Vec::remove(0) which is O(n).",
                "Interview tip: for shortest path questions, reach for BFS first. For connected-components / cycle detection, DFS.",
            ],
        },

        // ── 7. Heaps and Top-K ─────────────────────────────────────────────
        Lesson {
            id: "heaps-and-top-k",
            category: "Interview Algorithms",
            title: "Heaps and Top-K",
            description: r#"<p><code>std::collections::BinaryHeap</code> is a <b>max-heap</b>. The largest element is always at the top. Push and pop are <b>O(log n)</b>; peek is <b>O(1)</b>.</p>
<h3>Min-heap</h3>
<p>Wrap elements in <code>std::cmp::Reverse(x)</code> to invert the ordering. <code>BinaryHeap&lt;Reverse&lt;T&gt;&gt;</code> becomes a min-heap.</p>
<h3>Top-K smallest (classic interview pattern)</h3>
<p>Keep a max-heap of size k. When the heap exceeds k, pop the maximum — you're left with the k smallest elements seen so far. Time: <b>O(n log k)</b>. Space: <b>O(k)</b>. Better than sorting (O(n log n)) when k &lt;&lt; n.</p>
<h3>Other heap patterns</h3>
<ul>
  <li>K-way merge: merge k sorted lists using a min-heap — O(n log k)</li>
  <li>Median of a stream: two heaps (max-heap for lower half, min-heap for upper half)</li>
  <li>Dijkstra's shortest path: min-heap of (distance, node)</li>
</ul>"#,
            code: r#"use std::collections::BinaryHeap;
use std::cmp::Reverse;

// Top-k smallest elements from a slice.
// Strategy: max-heap of size k; when exceeded, pop the largest.
// After processing all n elements, the heap contains the k smallest.
// Time: O(n log k)  Space: O(k)
fn top_k_smallest(nums: &[i32], k: usize) -> Vec<i32> {
    let mut heap: BinaryHeap<i32> = BinaryHeap::new(); // max-heap
    for &n in nums {
        heap.push(n);
        if heap.len() > k {
            heap.pop(); // evict the current maximum
        }
    }
    let mut result: Vec<i32> = heap.into_vec();
    result.sort(); // heap doesn't guarantee order among the k elements
    result
}

// Kth largest using the same technique (min-heap of size k).
fn kth_largest(nums: &[i32], k: usize) -> Option<i32> {
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new(); // min-heap
    for &n in nums {
        heap.push(Reverse(n));
        if heap.len() > k {
            heap.pop(); // evict the current minimum
        }
    }
    heap.peek().map(|&Reverse(v)| v)
}

fn main() {
    let nums = vec![7, 10, 4, 3, 20, 15, 1, 8, 6, 2];
    println!("Input: {:?}", nums);

    for k in [1, 3, 5] {
        println!("Top-{} smallest : {:?}", k, top_k_smallest(&nums, k));
        println!("{}th largest     : {:?}", k, kth_largest(&nums, k));
    }

    // --- Max-heap basics ---
    println!("\n--- BinaryHeap (max-heap) pop order ---");
    let mut max_heap: BinaryHeap<i32> = [5, 1, 8, 3, 7].iter().cloned().collect();
    while let Some(v) = max_heap.pop() {
        print!("{} ", v);
    }
    println!("(largest first)");

    // --- Min-heap using Reverse ---
    println!("--- BinaryHeap<Reverse<i32>> (min-heap) pop order ---");
    let mut min_heap: BinaryHeap<Reverse<i32>> = [5, 1, 8, 3, 7]
        .iter()
        .map(|&x| Reverse(x))
        .collect();
    while let Some(Reverse(v)) = min_heap.pop() {
        print!("{} ", v);
    }
    println!("(smallest first)");
}"#,
            notes: vec![
                "BinaryHeap is a max-heap. Wrap in Reverse<T> for a min-heap — it inverts Ord automatically.",
                "Top-k smallest: max-heap of size k. Pop the max when full. O(n log k) beats sorting's O(n log n) when k << n.",
                "BinaryHeap::peek() is O(1) — it only reads the top without removing it.",
                "Dijkstra's: push (Reverse(dist), node) into a min-heap. Always best with a min-heap.",
                "Interview tip: know BOTH heap orientations and which one to use for top-k-smallest vs top-k-largest.",
            ],
        },

        // ── 8. Linked Lists ────────────────────────────────────────────────
        Lesson {
            id: "linked-list",
            category: "Interview Algorithms",
            title: "Linked Lists in Rust",
            description: r#"<p>Singly-linked lists are straightforward in safe Rust using <code>Option&lt;Box&lt;Node&lt;T&gt;&gt;&gt;</code> for the <code>next</code> pointer. Each node owns the next node via a <code>Box</code>; an empty link is <code>None</code>.</p>
<h3>Doubly-linked lists are hard in safe Rust</h3>
<p>They require shared mutable references: a node must be pointed at by both its predecessor and its successor. In safe Rust this requires <code>Rc&lt;RefCell&lt;Node&gt;&gt;</code> or <code>Arc&lt;Mutex&lt;Node&gt;&gt;</code>, which adds overhead. A hand-rolled doubly-linked list with raw pointers (<code>*mut Node</code>) uses <code>unsafe</code> — this is how the standard <code>LinkedList</code> is implemented.</p>
<h3>When to use LinkedList in interviews</h3>
<ul>
  <li>If the interviewer asks for it explicitly.</li>
  <li>When O(1) arbitrary insert/delete is required AND you have a pointer to the position.</li>
  <li>Otherwise, <code>Vec</code> or <code>VecDeque</code> are almost always faster in practice (cache locality).</li>
</ul>
<p><code>std::collections::LinkedList</code> exists but the Rust docs note it is "almost always worse than Vec or VecDeque".</p>"#,
            code: r#"// Singly-linked list using Option<Box<Node<T>>> — safe Rust, no unsafe.
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    value: T,
    next:  Link<T>,
}

struct LinkedList<T> {
    head: Link<T>,
    len:  usize,
}

impl<T: std::fmt::Display> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None, len: 0 }
    }

    // O(1) push to the front
    fn push_front(&mut self, value: T) {
        let node = Box::new(Node {
            value,
            next: self.head.take(), // steal the old head
        });
        self.head = Some(node);
        self.len += 1;
    }

    // O(1) pop from the front
    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.len -= 1;
            node.value
        })
    }

    // O(n) peek at the nth element (0-indexed)
    fn peek(&self) -> Option<&T> {
        self.head.as_deref().map(|n| &n.value)
    }

    fn len(&self) -> usize { self.len }
    fn is_empty(&self) -> bool { self.len == 0 }

    fn print_all(&self) {
        let mut cur = &self.head;
        let mut parts = Vec::new();
        while let Some(node) = cur {
            parts.push(format!("{}", node.value));
            cur = &node.next;
        }
        println!("[{}]", parts.join(" → "));
    }
}

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();
    for i in [3, 2, 1] {
        list.push_front(i);
    }
    print!("List (len={}): ", list.len());
    list.print_all();

    println!("peek  : {:?}", list.peek());
    println!("pop   : {:?}", list.pop_front());
    println!("pop   : {:?}", list.pop_front());
    print!("After 2 pops: ");
    list.print_all();

    // Drain the rest
    while let Some(v) = list.pop_front() {
        print!("popped {} ", v);
    }
    println!("\nEmpty: {}", list.is_empty());
}"#,
            notes: vec![
                "Option<Box<Node<T>>> is the idiomatic singly-linked list in safe Rust. Each node owns the next.",
                "Doubly-linked lists need shared mutable refs — use Rc<RefCell<Node>> in safe Rust or unsafe raw pointers.",
                "std::collections::LinkedList exists but is almost always slower than Vec/VecDeque due to cache misses.",
                "push_front is O(1); pop_front is O(1); arbitrary insert/delete requires O(n) traversal (no iterator-based cursors in stable Rust).",
                "Interview tip: if asked to implement a linked list, start with the singly-linked safe version and discuss doubly-linked trade-offs.",
            ],
        },

        // ── 9. LRU Cache ───────────────────────────────────────────────────
        Lesson {
            id: "lru-cache",
            category: "Interview Algorithms",
            title: "LRU Cache (Design Classic)",
            description: r#"<p>An <b>LRU (Least Recently Used) cache</b> evicts the least recently accessed entry when it exceeds capacity. The canonical O(1) implementation uses a <b>HashMap + doubly-linked list</b>: the map gives O(1) lookup; the list maintains recency order with O(1) move-to-front.</p>
<h3>Trade-offs by implementation</h3>
<ul>
  <li><b>HashMap + doubly-linked list</b>: O(1) get and put. Hard to implement in safe Rust (shared mutable refs). Requires <code>unsafe</code> raw pointers or <code>Rc&lt;RefCell&lt;Node&gt;&gt;</code>.</li>
  <li><b>HashMap + tick counter</b> (shown here): O(1) get, <b>O(n) put</b> when evicting. Simple, fully safe Rust. Fine for interviews when asked to sketch the idea — explain the trade-off.</li>
  <li><b>Crates</b>: the <code>lru</code> crate provides a production-quality O(1) implementation.</li>
</ul>
<p>In interviews: implement the tick version first, state the complexity clearly, then explain how you'd get O(1) eviction with a doubly-linked list and HashMap combo.</p>"#,
            code: r#"use std::collections::HashMap;

// Simple LRU using HashMap + monotonic tick counter.
// get: O(1)   put: O(1) amortised except O(n) eviction scan when full.
struct LruCache {
    capacity: usize,
    map:  HashMap<i32, (i32, u64)>, // key → (value, last_access_tick)
    tick: u64,
}

impl LruCache {
    fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be positive");
        LruCache { capacity, map: HashMap::new(), tick: 0 }
    }

    // Returns Some(value) and refreshes the entry's tick. O(1).
    fn get(&mut self, key: i32) -> Option<i32> {
        self.tick += 1;
        let t = self.tick;
        self.map.get_mut(&key).map(|entry| {
            entry.1 = t; // update recency
            entry.0
        })
    }

    // Inserts or updates a key. Evicts LRU if at capacity. O(n) eviction.
    fn put(&mut self, key: i32, value: i32) {
        self.tick += 1;
        if let Some(entry) = self.map.get_mut(&key) {
            entry.0 = value;
            entry.1 = self.tick;
            return;
        }
        if self.map.len() == self.capacity {
            // Find the key with the smallest tick — O(n) linear scan.
            let mut min_tick = u64::MAX;
            let mut evict_key = 0i32;
            for (&k, &(_, t)) in &self.map {
                if t < min_tick {
                    min_tick = t;
                    evict_key = k;
                }
            }
            self.map.remove(&evict_key);
        }
        self.map.insert(key, (value, self.tick));
    }
}

fn main() {
    let mut cache = LruCache::new(3);

    cache.put(1, 10);
    cache.put(2, 20);
    cache.put(3, 30);

    println!("get(1) = {:?}  (refreshes key 1)", cache.get(1)); // Some(10)

    cache.put(4, 40); // evicts key 2 (LRU — tick 2, while 1 was refreshed)
    println!("get(2) = {:?}  (evicted)", cache.get(2));          // None
    println!("get(3) = {:?}", cache.get(3));                     // Some(30)
    println!("get(4) = {:?}", cache.get(4));                     // Some(40)

    cache.put(5, 50); // cache is full (keys 1,3,4); evicts LRU
    println!("get(1) = {:?}", cache.get(1)); // depends on tick order
    println!("get(5) = {:?}", cache.get(5)); // Some(50)

    println!("\nFor O(1) put: use HashMap + doubly-linked list.");
    println!("In production Rust, reach for the `lru` crate.");
}"#,
            notes: vec![
                "Canonical O(1) LRU: HashMap for O(1) lookup + doubly-linked list for O(1) move-to-front on access.",
                "Tick-counter version shown here: O(1) get, O(n) put (eviction scan). State this trade-off explicitly.",
                "In safe Rust, the doubly-linked list requires Rc<RefCell<Node>> — the lru crate uses unsafe raw pointers.",
                "LinkedHashMap from the indexmap crate is another practical O(1) option if external crates are allowed.",
                "Interview tip: sketch the HashMap + linked-list design on a whiteboard, state O(1) for both ops, then code the simpler tick version.",
            ],
        },
    ]
}
