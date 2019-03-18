using UnityEngine;
using System.Collections;

public class Chunk
{
    protected readonly Block[,,] blocks = new Block[Constants.ChunkSize,Constants.ChunkLayers,Constants.ChunkSize];

    public World World { get; private set; }
    public BlockPos Position { get; private set; }

    public Chunk(World world, BlockPos position)
    {
        World = world;
        Position = position;
    }

    /// <summary>
    /// Gets and returns a block from a position within the chunk or fetches it from the world.
    /// </summary>
    /// <param name="blockPos">A global block position</param>
    /// <returns>The block at the position</returns>
    public Block Get(BlockPos blockPos)
    {
        if(InRange(blockPos))
        {
            return LocalGet(blockPos - Position);
        }
        else
        {
            return World.Blocks.Get(blockPos);
        }
    }

    /// <summary>
    /// This function takes a block position relative to the chunks position. It is slightly faster
    /// than the Get function to use this if you already have a local position available, otherwise
    /// simply use Get. If the position is less- or greater-than the size of the chunk it will get
    /// the value from the chunk containing the block pos.
    /// </summary>
    /// <param name="localBlockPos"></param>
    /// <returns></returns>
    public Block LocalGet(BlockPos localBlockPos)
    {
        if(localBlockPos.x < Constants.ChunkSize && localBlockPos.x >= 0 &&
            localBlockPos.y < Constants.ChunkLayers && localBlockPos.y >= 0 &&
            localBlockPos.z < Constants.ChunkSize && localBlockPos.z >= 0)
        {
            return blocks[localBlockPos.x, localBlockPos.y, localBlockPos.z];
        }
        else
        {
            return World.Blocks.Get(localBlockPos + Position);
        }
    }

    /// <summary>
    /// Returns true if theposition is contained in the chunk boundaries.
    /// </summary>
    /// <param name="blockPos"></param>
    public bool InRange(BlockPos blockPos)
    {
        return blockPos.ContainingChunkCoordinates() == Position;
    }

    public void Set(BlockPos blockPos, Block block)
    {
        if(InRange(blockPos))
        {
            BlockPos localPos = blockPos - Position;
            blocks[localPos.x, localPos.y, localPos.z] = block;
        }
        else
        {
            World.Blocks.Set(blockPos, block);
        }
    }
}
