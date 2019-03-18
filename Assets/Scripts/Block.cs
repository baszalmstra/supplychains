using UnityEngine;
using System.Collections;

public enum Block : ushort 
{
    // Represents uninitialized block data
    Void    = 0x0000,

    // Empty space
    Air     = 0x0001,

    // Represents whatever is at the lowers of the lowest level. This is a non-transparent block so 
    // that the bottom of chunks is not visible.
    BedRock = 0x0003,

    // First usable block type
    Grass   = 0x0002,
}

public static class BlockExtensions
{
    public static bool IsTransparent(this Block block)
    {
        return block <= Block.Air;
    }
}
