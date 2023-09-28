#ifndef DEF_CUH
#define DEF_CUH

#include <stdint.h>
#include <cassert>
#include <stdio.h>
#include <iostream>

//#define PRINT_HEX(data) \
//    do  {               \
//        printf("{");                \
//        for (int k = 0; k < sizeof(data); ++k) printf("0x%02x%s", ((uint8_t*)&(data))[k], k==sizeof(data)-1?"":", ");\
//        printf("}\n");\
//    } while(0)

#define PRINT_HEX_2(PROMT, ARR, N, BUF)					\
  do {									\
    int __my_local_remain = N;						\
    __my_local_remain -= snprintf(BUF, __my_local_remain, "%s: ", PROMT); \
    for (size_t __i_idx = 0; __i_idx < sizeof(ARR); __i_idx++) {	\
      __my_local_remain -= snprintf(BUF, __my_local_remain, "%02x", ((uint8_t*)&(ARR))[__i_idx]); \
      if ((__i_idx + 1) % 8 == 0 && __i_idx != sizeof(ARR) - 1) {	\
	__my_local_remain -= snprintf(BUF, __my_local_remain, ", ");	\
      }									\
    }									\
    snprintf(BUF, n, "\n");						\
  }while(0)

#define PRINT_HEX(PROMT, ARR)						\
  do {									\
    printf("%s: ", PROMT);						\
    for (size_t __i_idx = 0; __i_idx < sizeof(ARR); __i_idx++) {	\
      printf("%02x", ((uint8_t*)&(ARR))[__i_idx]);			\
      if ((__i_idx + 1) % 8 == 0 && __i_idx != sizeof(ARR) - 1) {	\
	printf(", ");							\
      }									\
    }									\
    printf("\n");							\
  }while(0)

typedef uint32_t u32;
typedef uint64_t u64;
typedef unsigned __int128 u128;
typedef __int128 i128;
typedef size_t usize;

static inline __device__ int get_global_id() {
    const int gid = threadIdx.x + blockIdx.x * blockDim.x;
    return gid;
}
static inline __device__ int get_global_thcnt()
{
    return gridDim.x * blockDim.x;
}

static inline __device__ uint64_t overflowing_add(uint64_t a, uint64_t b, int* overflow) {
    *overflow = UINT64_MAX - b < a;
    return a + b;
}
static inline __device__ uint64_t overflowing_sub(uint64_t a, uint64_t b, int* overflow) {
    *overflow = a < b;
    return a - b;
}

const uint64_t EPSILON = (1ULL << 32) - 1;

template<int BYTES>
struct __align__(8) bytes_pad_type {
uint8_t data[BYTES];
};

#define BYTES_ASSIGN(dst, src, len)  \
        *(bytes_pad_type<len>*)(dst) = *(bytes_pad_type<len>*)(src)

#if 0
class u128 {
public:
    uint64_t low;
    uint64_t high;

    __device__ inline u128(uint64_t l = 0, uint64_t h = 0) : low(l), high(h) {}

    __device__ inline u128 operator+(const u128& other) const {
        uint64_t sum_low = low + other.low;
        uint64_t carry = sum_low < low ? 1 : 0;
        uint64_t sum_high = high + other.high + carry;
        return u128(sum_low, sum_high);
    }

    __device__ inline u128 operator-(const u128& other) const {
        uint64_t diff_low = low - other.low;
        uint64_t borrow = diff_low > low ? 1 : 0;
        uint64_t diff_high = high - other.high - borrow;
        return u128(diff_low, diff_high);
    }

    __device__ inline u128 operator*(const u128& other) const {
//        uint64_t a0 = low & 0xFFFFFFFF;
//        uint64_t a1 = low >> 32;
//        uint64_t b0 = other.low & 0xFFFFFFFF;
//        uint64_t b1 = other.low >> 32;
//
//        uint64_t prod0 = a0 * b0;
//        uint64_t prod1 = a1 * b0 + (prod0 >> 32);
//        uint64_t prod2 = a0 * b1 + (prod1 & 0xFFFFFFFF);
//        uint64_t prod3 = a1 * b1 + (prod2 >> 32);
//
//        uint64_t carry = (prod3 >> 32) + (prod2 >> 32) + (prod1 >> 32);
//        uint64_t result_low = (prod0 & 0xFFFFFFFF) | (prod1 << 32);
//        uint64_t result_high = prod3 + carry;
//
//        return u128{result_low, result_high};
        auto b = other;
        auto a = *this;

        u128 result = {0, 0};
        for (int i = 0; i < 64; i++) {
            if (b.low & 1) {
                result = result + a;
            }
            a = a << 1;
            b.low >>= 1;
        }
        return result;

    }

    __device__ inline u128& operator+=(const u128& other) {
        *this = *this + other;
        return *this;
    }

    __device__ inline u128 operator>>(int shift) const {
        if (shift >= 128) {
            return u128(0, 0);
        } else if (shift >= 64) {
            return u128(high >> (shift - 64), 0);
        } else {
            return u128((low >> shift) | (high << (64 - shift)), high >> shift);
        }
    }

    __device__ inline u128 operator<<(int shift) const {
        u128 result;
        if (shift >= 64) {
            result.high = this->low << (shift - 64);
            result.low = 0;
        } else {
            result.high = (this->high << shift) | (this->low >> (64 - shift));
            result.low = this->low << shift;
        }
        return result;
        return result;
    }

    __device__ inline u128 overflowing_add(const u128& other, bool* overflow) const {
        u128 result = *this + other;
        *overflow = (result.high < high) || ((result.high == high) && (result.low < low));
        return result;
    }

    __device__ inline operator uint64_t() const {
        return low;
    }

};
#endif

struct  GoldilocksField{
    uint64_t data;
    static const uint64_t TWO_ADICITY = 32;
    static const uint64_t CHARACTERISTIC_TWO_ADICITY= TWO_ADICITY;

    static const uint64_t ORDER = 0xFFFFFFFF00000001;

#define from_canonical_usize from_canonical_u64

    __device__ inline
    static const GoldilocksField coset_shift() {
        return GoldilocksField{7};
    }

    __device__ inline GoldilocksField square() const {
        return (*this) * (*this);
    }
    __device__ inline GoldilocksField sub_one() {
        return (*this) - from_canonical_u64(1);
    }

    __device__ inline uint64_t to_noncanonical_u64() const{
        return this->data;
    }

    static __device__ inline GoldilocksField from_canonical_u64(uint64_t n) {
        return GoldilocksField{n};
    }

    static __device__ inline GoldilocksField from_noncanonical_u96(uint64_t n_lo, uint32_t n_hi) {
        // Default implementation.
        u128 n = (u128(n_hi) << 64) + u128(n_lo);
        return from_noncanonical_u128(n);
    }

    static __device__ inline GoldilocksField  from_noncanonical_u128(u128 n) {
        return reduce128(n >> 64, n & UINT64_MAX);
    }

    __device__ inline GoldilocksField inverse() const {
        u64 f = this->data;
        u64 g = GoldilocksField::ORDER;
        // NB: These two are very rarely such that their absolute
        // value exceeds (p-1)/2; we are paying the price of i128 for
        // the whole calculation, just for the times they do
        // though. Measurements suggest a further 10% time saving if c
        // and d could be replaced with i64's.
        i128 c = 1;
        i128 d = 0;

//        assert (f != 0);

        auto trailing_zeros = [](uint64_t n) -> int{
            int count = 0;
            while ((n & 1) == 0) {
                n >>= 1;
                count++;
            }
            return count;
        };


// f and g must always be odd.
        u32  k = trailing_zeros(f);
        f >>= k;
        if (f == 1) {
            return GoldilocksField::inverse_2exp(k);
        }

        // The first two iterations are unrolled. This is to handle
        // the case where f and g are both large and f+g can
        // overflow. log2(max{f,g}) goes down by at least one each
        // iteration though, so after two iterations we can be sure
        // that f+g won't overflow.
        auto swap = [](auto& a, auto& b) {
            auto temp = a;
            a = b;
            b = temp;
        };

        auto safe_iteration = [trailing_zeros, swap](u64& f, u64& g, i128& c, i128& d, u32& k) {
            if (f < g) {
                swap(f, g);
                swap(c, d);
            }
            if ((f & 3) == (g & 3)) {
                // f - g = 0 (mod 4)
                f -= g;
                c -= d;

                // kk >= 2 because f is now 0 (mod 4).
                auto kk = trailing_zeros(f);
                f >>= kk;
                d <<= kk;
                k += kk;
            } else {
                // f + g = 0 (mod 4)
                f = (f >> 2) + (g >> 2) + 1ULL;
                c += d;
                auto kk = trailing_zeros(f);
                f >>= kk;
                d <<= kk + 2;
                k += kk + 2;
            }
        };

        // Iteration 1:
        safe_iteration(f, g, c, d, k);

        if (f == 1) {
            // c must be -1 or 1 here.
            if (c == -1) {
                return -GoldilocksField::inverse_2exp(k);
            }
            assert(c == 1);
            return GoldilocksField::inverse_2exp(k);
        }

        // Iteration 2:
        safe_iteration(f, g, c, d, k);


        auto unsafe_iteration = [trailing_zeros, swap](u64& f, u64& g, i128& c, i128& d, u32& k) {
            if (f < g) {
                swap(f, g);
                swap(c, d);
            }
            if ((f & 3) == (g & 3)) {
                // f - g = 0 (mod 4)
                f -= g;
                c -= d;
            } else {
                // f + g = 0 (mod 4)
                f += g;
                c += d;
            }

            // kk >= 2 because f is now 0 (mod 4).
            auto kk = trailing_zeros(f);
            f >>= kk;
            d <<= kk;
            k += kk;
        };

        // Remaining iterations:
        while (f != 1) {
            unsafe_iteration(f, g, c, d, k);
        }

        // The following two loops adjust c so it's in the canonical range
        // [0, F::ORDER).

        // The maximum number of iterations observed here is 2; should
        // prove this.
        while (c < 0) {
            c += i128(GoldilocksField::ORDER);
        }

        // The maximum number of iterations observed here is 1; should
        // prove this.
        while (c >= i128(GoldilocksField::ORDER)) {
            c -= i128(GoldilocksField::ORDER);
        }

        // Precomputing the binary inverses rather than using inverse_2exp
        // saves ~5ns on my machine.
        auto res = GoldilocksField::from_canonical_u64(u64(c)) * GoldilocksField::inverse_2exp(u64(k));
//        assert(*this * res == GoldilocksField::from_canonical_u64(1));
        return res;
    }


    __device__ inline GoldilocksField inverse_2exp(u64 exp) const {
        // Let p = char(F). Since 2^exp is in the prime subfield, i.e. an
        // element of GF_p, its inverse must be as well. Thus we may add
        // multiples of p without changing the result. In particular,
        // 2^-exp = 2^-exp - p 2^-exp
        //        = 2^-exp (1 - p)
        //        = p - (p - 1) / 2^exp

        // If this field's two adicity, t, is at least exp, then 2^exp divides
        // p - 1, so this division can be done with a simple bit shift. If
        // exp > t, we repeatedly multiply by 2^-t and reduce exp until it's in
        // the right range.

//        if let Some(p) = Self::characteristic().to_u64() {
        if (true) {
            auto p = GoldilocksField::ORDER;
            // NB: The only reason this is split into two cases is to save
            // the multiplication (and possible calculation of
            // inverse_2_pow_adicity) in the usual case that exp <=
            // TWO_ADICITY. Can remove the branch and simplify if that
            // saving isn't worth it.

            if (exp > GoldilocksField::CHARACTERISTIC_TWO_ADICITY) {
                // NB: This should be a compile-time constant
                auto inverse_2_pow_adicity =
                        GoldilocksField::from_canonical_u64(p - ((p - 1) >> GoldilocksField::CHARACTERISTIC_TWO_ADICITY));

                auto res = inverse_2_pow_adicity;
                auto e = exp - GoldilocksField::CHARACTERISTIC_TWO_ADICITY;

                while (e > GoldilocksField::CHARACTERISTIC_TWO_ADICITY) {
                    res *= inverse_2_pow_adicity;
                    e -= GoldilocksField::CHARACTERISTIC_TWO_ADICITY;
                }
                return res * GoldilocksField::from_canonical_u64(p - ((p - 1) >> e));
            } else {
                return GoldilocksField::from_canonical_u64(p - ((p - 1) >> exp));
            }
        } else {
            return GoldilocksField::from_canonical_u64(2).inverse().exp_u64(exp);
        }
    }

    __device__ inline
    GoldilocksField exp_u64(u64 power) const {
        auto current = *this;
        auto product = GoldilocksField::from_canonical_u64(1);

        for (int j = 0; j < 64; ++j) {
            if (((power >> j) & 1) != 0) {
                product *= current;
            }
            current = current.square();
        }
        return product;
    }

    __device__ inline
    GoldilocksField operator+(const GoldilocksField& rhs) const {
        int over = 0;
        uint64_t sum = overflowing_add(this->data, rhs.data, &over);
        sum = overflowing_add(sum, over * EPSILON, &over);
        if (over) {
            // NB: self.0 > Self::ORDER && rhs.0 > Self::ORDER is necessary but not sufficient for
            // double-overflow.
            // This assume does two things:
            //  1. If compiler knows that either self.0 or rhs.0 <= ORDER, then it can skip this
            //     check.
            //  2. Hints to the compiler how rare this double-overflow is (thus handled better with
            //     a branch).
            assert(this->data > GoldilocksField::ORDER && rhs.data > GoldilocksField::ORDER);
//                    branch_hint();
            sum += EPSILON; // Cannot overflow.
        }
        return GoldilocksField{.data = sum};
    }
    __device__ inline
    GoldilocksField operator-(const GoldilocksField& rhs) const {
        int under = 0;
        uint64_t diff = overflowing_sub(this->data, rhs.data, &under);
        diff = overflowing_sub(diff, under * EPSILON, &under);
        if (under) {
            // NB: self.0 > Self::ORDER && rhs.0 > Self::ORDER is necessary but not sufficient for
            // double-overflow.
            // This assume does two things:
            //  1. If compiler knows that either self.0 or rhs.0 <= ORDER, then it can skip this
            //     check.
            //  2. Hints to the compiler how rare this double-overflow is (thus handled better with
            //     a branch).
            assert(this->data < EPSILON - 1 && rhs.data > GoldilocksField::ORDER);
//                    branch_hint();
            diff -= EPSILON; // Cannot overflow.
        }
        return GoldilocksField{.data = diff};
    }

    static __device__ inline
    GoldilocksField reduce128(uint64_t x_hi, uint64_t x_lo) {
        uint64_t x_hi_hi = x_hi >> 32;
        uint64_t x_hi_lo = x_hi & EPSILON;

        int borrow = 0;
        uint64_t t0 = overflowing_sub(x_lo, x_hi_hi, &borrow);
        if (borrow) {
//            branch_hint(); // A borrow is exceedingly rare. It is faster to branch.
            t0 -= EPSILON; // Cannot underflow.
        }
        uint64_t t1 = x_hi_lo * EPSILON;
//        uint64_t t2 = unsafe { add_no_canonicalize_trashing_input(t0, t1) };

        uint64_t t2;
        if (UINT64_MAX - t1 < t0) {
            t2 = (t1 + t0) + (0xffffffff);
        }
        else
            t2 = (t0 + t1);
        return GoldilocksField{.data = t2};
    }

    __device__ inline
    GoldilocksField operator*(const GoldilocksField& rhs) const {
        uint64_t high, low, a = this->data, b = rhs.data;
        {
            uint64_t a_low = a & 0xFFFFFFFF;
            uint64_t a_high = a >> 32;
            uint64_t b_low = b & 0xFFFFFFFF;
            uint64_t b_high = b >> 32;

            uint64_t product_low = a_low * b_low;
            uint64_t product_mid1 = a_low * b_high;
            uint64_t product_mid2 = a_high * b_low;
            uint64_t product_high = a_high * b_high;

            uint64_t carry = (product_low >> 32) + (product_mid1 & 0xFFFFFFFF) + (product_mid2 & 0xFFFFFFFF);
            high = product_high + (product_mid1 >> 32) + (product_mid2 >> 32) + (carry >> 32);
            low = (carry << 32) + (product_low & 0xFFFFFFFF);
        }
        return reduce128(high, low);
    }
    __device__ inline
    GoldilocksField& operator*=(const GoldilocksField& rhs) {
        *this = *this * rhs;
        return *this;
    }
    __device__ inline
    GoldilocksField& operator+=(const GoldilocksField& rhs) {
        *this = *this + rhs;
        return *this;
    }
    __device__ inline
    bool operator==(const GoldilocksField& rhs) {
        return rhs.data == this->data;
    }

    __device__ inline
    GoldilocksField operator-() {
        return GoldilocksField{-this->data};
    }

    __device__ inline
    GoldilocksField multiply_accumulate(GoldilocksField x, GoldilocksField y) {
        // Default implementation.
        return *this + x * y;
    }

    __device__ inline
    GoldilocksField add_canonical_u64(uint64_t rhs) {
        // Default implementation.
        return *this + GoldilocksField::from_canonical_u64(rhs);
    }

};

#include "constants.cuh"

template<class T1, class T2>
struct my_pair {
    T1 first;
    T2 second;
    __device__ inline my_pair(const T1& t1, const T2& t2)
            :first(t1), second(t2)
    {
    }
};

template <class T>
struct Range :my_pair<T, T> {
    __device__ inline Range(const T& t1, const T& t2)
            :my_pair<T, T>(t1, t2)
    {
    }

    struct Iterator {
        using iterator_category = std::forward_iterator_tag;
        using value_type = T;

        // 构造函数
        __device__ inline
        Iterator(value_type p) :num(p) {}

        // 拷贝赋值函数
        __device__ inline
        Iterator& operator=(const Iterator& it) {
            num = it.num;
        }

        // 等于运算符
        __device__ inline
        bool operator==(const Iterator& it) const {
            return num == it.num;
        }

        // 不等于运算符
        __device__ inline
        bool operator!=(const Iterator& it) const {
            return num != it.num;
        }

        // 前缀自加
        __device__ inline
        Iterator& operator++() {
            num++;
            return *this;
        }

        // 后缀自加
        __device__ inline
        Iterator operator ++(int) {
            Iterator tmp = *this;
            ++(*this);
            return tmp;
        }

        // 前缀自减
        __device__ inline
        Iterator& operator--() {
            num--;
            return *this;
        }

        // 后缀自减
        __device__ inline
        Iterator operator --(int) {
            Iterator tmp = *this;
            --(*this);
            return tmp;
        }

//        // 取值运算
        __device__ inline
        value_type & operator*() {
            return num;
        }

    private:
        // 定义一个指针
        value_type num;
    };


    // 遍历的第一个元素的位置
    __device__ inline
    Iterator begin() {
        return Iterator(this->first);
    }

    // 遍历的最后一个元素的下一个位置
    __device__ inline
    Iterator end() {
        return Iterator(this->second);
    }

};

struct PoseidonHasher {
    struct HashOut {
        GoldilocksField elements[4] ;
    };

    static __device__ inline my_pair<u128, u32> add_u160_u128(my_pair<u128, u32> pa, u128 y) {
        auto x_lo = pa.first;
        auto x_hi = pa.second;

        auto overflowing_add = [](u128 a, u128 b, bool* overflow) {
            *overflow = ~__uint128_t{} - b < a;
            return a + b;
        };

        bool over;
        auto res_lo = overflowing_add(x_lo, y, &over);
        u32 res_hi = x_hi + u32(over);
        return my_pair<u128, u32>{res_lo, res_hi};
    }

    static __device__ inline GoldilocksField reduce_u160(my_pair<u128, u32> pa) {
        auto n_lo = pa.first;
        auto n_hi = pa.second;

        u64 n_lo_hi = (n_lo >> 64);
        u64 n_lo_lo = n_lo;
        u64 reduced_hi = GoldilocksField::from_noncanonical_u96(n_lo_hi, n_hi).to_noncanonical_u64();
        u128 reduced128 = (u128(reduced_hi) << 64) + u128(n_lo_lo);
        return GoldilocksField::from_noncanonical_u128(reduced128);
    }

    static __device__ inline void print_state(const char* promt, GoldilocksField* state) {
        printf("%s: [", promt);
        for (int i = 0; i < 12; ++i) {
            printf("%lu%s", state[i].data, i == 11?"]\n":", ");
        }
    }
    static __device__ inline
    void permute_poseidon(GoldilocksField* state) {
        int round_ctr = 0;

        constexpr int WIDTH = SPONGE_WIDTH;
        auto constant_layer = [&]() {
            for (int i = 0; i < 12; ++i) {
                if (i < WIDTH) {
                    uint64_t round_constant = ALL_ROUND_CONSTANTS[i + WIDTH * round_ctr];
                    state[i] = state[i].add_canonical_u64(round_constant);
                }
            }
        };

        auto sbox_monomial = [](GoldilocksField x) -> GoldilocksField {
            // x |--> x^7
            GoldilocksField x2 = x.square();
            GoldilocksField x4 = x2.square();
            GoldilocksField x3 = x * x2;
            return x3 * x4;
        };

        auto sbox_layer = [&]() {
            for (int i = 0; i < 12; ++i) {
                if (i < WIDTH) {
                    state[i] = sbox_monomial(state[i]);
                }
            }
        };

        auto mds_row_shf = [](int r, uint64_t v[WIDTH]) -> u128 {
            assert(r < WIDTH);
            // The values of `MDS_MATRIX_CIRC` and `MDS_MATRIX_DIAG` are
            // known to be small, so we can accumulate all the products for
            // each row and reduce just once at the end (done by the
            // caller).

            // NB: Unrolling this, calculating each term independently, and
            // summing at the end, didn't improve performance for me.
            u128 res = 0;

            // This is a hacky way of fully unrolling the loop.
            for (int i = 0; i < 12; ++i) {
                if (i < WIDTH) {
                    res += u128(v[(i + r) % WIDTH]) * u128(MDS_MATRIX_CIRC[i]);
//                    printf("state 1211: %lu, %lu\n", res.high, res.low);
                }
            }
            res += u128(v[r]) * u128(MDS_MATRIX_DIAG[r]);
            return res;
        };

        auto mds_layer = [&]() {
            uint64_t _state[SPONGE_WIDTH] = {0};

            for (int r = 0; r < WIDTH; ++r)
                _state[r] = state[r].to_noncanonical_u64();

            // This is a hacky way of fully unrolling the loop.
            for (int r = 0; r < 12; ++r) {
                if (r < WIDTH) {
                    auto sum = mds_row_shf(r, _state);
//                    printf("state 121: %lu, %lu\n", sum.high, sum.low);
                    uint64_t sum_lo = sum;
                    uint32_t sum_hi = (sum >> 64);
                    state[r] = GoldilocksField::from_noncanonical_u96(sum_lo, sum_hi);
//                    printf("state 122: %lu, lo: %lu, hi: %u\n", state[r].data, sum_lo, sum_hi);
                }
            }
        };

        auto full_rounds = [&]() {
            for (int r = 0; r < HALF_N_FULL_ROUNDS; ++r) {
                constant_layer();
//                print_state("state11", state);
                sbox_layer();
//                print_state("state12", state);
                mds_layer();
//                print_state("state13", state);
                round_ctr += 1;
            }
        };

        auto partial_first_constant_layer = [&]() {
            for (int i = 0; i < 12; ++i) {
                if (i < WIDTH) {
                    state[i] += GoldilocksField::from_canonical_u64(FAST_PARTIAL_FIRST_ROUND_CONSTANT[i]);
                }
            }
        };

        auto mds_partial_layer_init = [&]() {
            // Initial matrix has first row/column = [1, 0, ..., 0];

            GoldilocksField result[WIDTH] = {0};
            // c = 0
            result[0] = state[0];

            for (int r = 1; r < 12; ++r) {
                if (r < WIDTH) {
                    for (int c = 1; c < 12; ++c) {
                        if (c < WIDTH) {
                            // NB: FAST_PARTIAL_ROUND_INITIAL_MATRIX is stored in
                            // row-major order so that this dot product is cache
                            // friendly.
                            auto t = GoldilocksField::from_canonical_u64(
                                    FAST_PARTIAL_ROUND_INITIAL_MATRIX[r - 1][c - 1]
                            );
                            result[c] += state[r] * t;
                        }
                    }
                }
            }
            for (int i = 0; i < WIDTH; ++i)
                state[i] = result[i];
        };

        auto mds_partial_layer_fast = [&](int r) {
            // Set d = [M_00 | w^] dot [state]
//            print_state("state21", state);

            my_pair<u128, u32> d_sum = {0, 0}; // u160 accumulator
            for (int i = 1; i < 12; ++i) {
                if (i < WIDTH) {
                    u128 t = FAST_PARTIAL_ROUND_W_HATS[r][i - 1];
                    u128 si = state[i].to_noncanonical_u64();
                    d_sum = add_u160_u128(d_sum, si * t);
                }
            }

            u128 s0 = u128(state[0].to_noncanonical_u64());
            u128 mds0to0 = u128(MDS_MATRIX_CIRC[0] + MDS_MATRIX_DIAG[0]);
            d_sum = add_u160_u128(d_sum, s0 * mds0to0);
            auto d = reduce_u160(d_sum);

            // result = [d] concat [state[0] * v + state[shift up by 1]]
            GoldilocksField result[SPONGE_WIDTH];
//            let mut result = [ZERO; WIDTH];
            result[0] = d;
            for (int i = 1; i < 12; ++i) {
                if (i < WIDTH) {
                    auto t = GoldilocksField::from_canonical_u64(FAST_PARTIAL_ROUND_VS[r][i - 1]);
                    result[i] = state[i].multiply_accumulate(state[0], t);
                }
            }
            for (int i = 0; i < 12; ++i)
                state[i] = result[i];
//            print_state("state22", state);
        };

        auto partial_rounds = [&]() {
            partial_first_constant_layer();
            mds_partial_layer_init();

            for (int i = 0; i < N_PARTIAL_ROUNDS; ++i) {
                state[0] = sbox_monomial(state[0]);
//            unsafe
                {
                    state[0] = state[0].add_canonical_u64(FAST_PARTIAL_ROUND_CONSTANTS[i]);
                }
//                *state = mds_partial_layer_fast(state, i);
                mds_partial_layer_fast(i);
            }
            round_ctr += N_PARTIAL_ROUNDS;
        };

//        print_state("state1", state);
        full_rounds();
//        print_state("state2", state);
        partial_rounds();
//        print_state("state3", state);
        full_rounds();
//        print_state("state4", state);

        assert(round_ctr == N_ROUNDS);

    }

    static __device__ inline HashOut hash_n_to_m_no_pad(const GoldilocksField* input) {
        GoldilocksField state[SPONGE_WIDTH] = {0};

        constexpr int len = 4;
        // Absorb all input chunks.
        for (int i = 0; i < len; i += SPONGE_RATE) {
            for (int j = 0; j < SPONGE_RATE; ++j)
                state[j] = input[i*SPONGE_RATE+j];
            permute_poseidon(state);
        }

        return *(HashOut*)state;
    }
};

struct GoldilocksFieldView {
    GoldilocksField* ptr;
    int len;

    __device__ inline
    GoldilocksFieldView view(int start, int end) const {
        return GoldilocksFieldView{this->ptr + start, end-start};
    }
    __device__ inline
    GoldilocksFieldView view(int start) const {
        return GoldilocksFieldView{this->ptr + start, this->len-start};
    }
    __device__ inline
    GoldilocksFieldView view(Range<int> range) const {
        return GoldilocksFieldView{this->ptr + range.first, range.second-range.first};
    }

    __device__ inline
    const GoldilocksField& operator[](int index) const {
        return this->ptr[index];
    }



    struct Iterator {
        using iterator_category = std::forward_iterator_tag;
        using difference_type = std::ptrdiff_t;
        using value_type = GoldilocksField;
        using reference = const GoldilocksField&;
        using pointer = GoldilocksField*;

        // 构造函数
        __device__ inline
        Iterator(pointer p) :ptr(p) {}

        // 拷贝赋值函数
        __device__ inline
        Iterator& operator=(const Iterator& it) {
            ptr = it.ptr;
        }

        // 等于运算符
        __device__ inline
        bool operator==(const Iterator& it) const {
            return ptr == it.ptr;
        }

        // 不等于运算符
        __device__ inline
        bool operator!=(const Iterator& it) const {
            return ptr != it.ptr;
        }

        // 前缀自加
        __device__ inline
        Iterator& operator++() {
            ptr++;
            return *this;
        }

        // 后缀自加
        __device__ inline
        Iterator operator ++(int) {
            Iterator tmp = *this;
            ++(*this);
            return tmp;
        }

        // 前缀自减
        __device__ inline
        Iterator& operator--() {
            ptr--;
            return *this;
        }

        // 后缀自减
        __device__ inline
        Iterator operator --(int) {
            Iterator tmp = *this;
            --(*this);
            return tmp;
        }

//        // 取值运算
        __device__ inline
        value_type & operator*() {
            return *ptr;
        }

    private:
        // 定义一个指针
        pointer ptr;
    };


    // 遍历的第一个元素的位置
    __device__ inline
    Iterator begin() {
        GoldilocksField* head = ptr;
        return Iterator(head);
    }

    // 遍历的最后一个元素的下一个位置
    __device__ inline
    Iterator end() {
        GoldilocksField* head = ptr;
        return Iterator(head + len);
    }


};


struct EvaluationVarsBasePacked {
    GoldilocksFieldView local_constants;
    GoldilocksFieldView local_wires;
    PoseidonHasher::HashOut public_inputs_hash;
};

struct StridedConstraintConsumer {
    GoldilocksField* terms;

    __device__ inline
    void one(GoldilocksField term) {
        *terms++ = term;
    }
};

template<class FN>
__device__ inline
GoldilocksField reduce_with_powers(Range<usize> range, FN f, GoldilocksField alpha)
{
    auto sum = GoldilocksField{0};
    for (int i = range.second-1; i >= 0; --i) {
        sum = sum * alpha + f(i);
    }
    return sum;
}

__device__ inline
GoldilocksField reduce_with_powers(GoldilocksFieldView terms, GoldilocksField alpha)
{
    return reduce_with_powers(Range<usize>{0, terms.len}, [terms](int i) ->GoldilocksField {
        return terms[i];
    }, alpha);
}

__device__ inline
static constexpr usize ceil_div_usize(usize a, usize b) {
    return (a + b - 1) / b;
}

#endif