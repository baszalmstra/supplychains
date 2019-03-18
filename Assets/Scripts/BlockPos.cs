﻿using UnityEngine;
using System.Collections;

public struct BlockPos
{
    public int x,y,z;

    public static readonly BlockPos zero = new BlockPos(0,0,0);
    public static readonly BlockPos one = new BlockPos(1,1,1);

    public BlockPos(int x, int y, int z)
    {
        this.x = x;
        this.y = y;
        this.z = z;
    }

    public override int GetHashCode()
    {
        unchecked
        {
            int hash = 47;
            hash = hash * 227 + x.GetHashCode();
            hash = hash * 227 + y.GetHashCode();
            hash = hash * 227 + z.GetHashCode();
            return hash * 227;
        }
    }

    public float Random(byte seed, bool extraPrecision = false)
    {
        int hash = GetHashCode();
        unchecked
        {
            hash *= primeNumbers[seed];
            if(extraPrecision)
            {
                hash *= GetHashCode() * primeNumbers[seed++];
                if(hash < 0)
                    hash *= -1;

                return (hash % 10000) / 10000.0f;
            }

            if(hash < 0)
                hash *= -1;

            return (hash % 100) / 100.0f;
        }
    }

    public static implicit operator BlockPos(BlockFace face)
    {
        switch (face)
        {
            case BlockFace.North: return new BlockPos(0,0,1);
            case BlockFace.East: return new BlockPos(1,0,0);
            case BlockFace.South: return new BlockPos(0,0,-1);
            case BlockFace.West: return new BlockPos(-1,0,0);
            case BlockFace.Up: return new BlockPos(0,1,0);
            case BlockFace.Down: return new BlockPos(0,-1,0);
            default:
                return new BlockPos();
        }
    }

    public BlockPos ContainingChunkCoordinates()
    {
        int divideDown(int a, int b)
        {
            return (a / b) + ((a % b) >> 31);
        }

        BlockPos result = new BlockPos(
            divideDown(x, Constants.ChunkSize) * Constants.ChunkSize,
            divideDown(y, Constants.ChunkLayers) * Constants.ChunkLayers,
            divideDown(z, Constants.ChunkSize) * Constants.ChunkSize);
        return result;
    }

    public BlockPos Add(int x, int y, int z)
    {
        return new BlockPos(this.x + x, this.y + y, this.z + z);
    }

    public BlockPos Add(BlockPos pos)
    {
        return new BlockPos(this.x + pos.x, this.y + pos.y, this.z + pos.z);
    }

    public BlockPos Subtract(BlockPos pos)
    {
        return new BlockPos(x - pos.x, y - pos.y, z - pos.z);
    }

    public BlockPos Negate()
    {
        return new BlockPos(-x, -y, -z);
    }

    public override bool Equals(object obj)
    {
        if (!(obj is BlockPos))
            return false;
        BlockPos other = (BlockPos)obj;
        return Equals(other);
    }

    public bool Equals(BlockPos other) 
    {
        if (GetHashCode() != other.GetHashCode())
            return false;
        if (x != other.x)
            return false;
        if (y != other.y)
            return false;
        if (z != other.z)
            return false;
        return true;
    }

    public static bool operator ==(BlockPos pos1, BlockPos pos2)
    {
        return pos1.Equals(pos2);
    }

    public static bool operator !=(BlockPos pos1, BlockPos pos2)
    {
        return !pos1.Equals(pos2);
    }

    public static BlockPos operator -(BlockPos lhs, BlockPos rhs)
    {
        return lhs.Subtract(rhs);
    }

    public static BlockPos operator +(BlockPos lhs, BlockPos rhs)
    {
        return lhs.Add(rhs);
    }

    //You can safely use BlockPos as part of a string like this:
    //"block at " + BlockPos + " is broken."
    public override string ToString()
    {
        return "(" + x + ", " + y + ", " + z + ")";
    }
    
    // first 255 prime numbers and 1. Used for randomizing a number in the Random function.
    static readonly int[] primeNumbers = new int[]{1, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997, 1009, 1013, 1019, 1021, 1031, 1033, 1039, 1049, 1051, 1061, 1063, 1069, 1087, 1091, 1093, 1097, 1103, 1109, 1117, 1123, 1129, 1151, 1153, 1163, 1171, 1181, 1187, 1193, 1201, 1213, 1217, 1223, 1229, 1231, 1237, 1249, 1259, 1277, 1279, 1283, 1289, 1291, 1297, 1301, 1303, 1307, 1319, 1321, 1327, 1361, 1367, 1373, 1381, 1399, 1409, 1423, 1427, 1429, 1433, 1439, 1447, 1451, 1453, 1459, 1471, 1481, 1483, 1487, 1489, 1493, 1499, 1511, 1523, 1531, 1543, 1549, 1553, 1559, 1567, 1571, 1579, 1583, 1597, 1601, 1607, 1609, 1613, 1619};
}
