using UnityEngine;
using System.Collections;

public class World : MonoBehaviour
{
    public WorldBlocks Blocks { get; private set; }
    public WorldChunks Chunks { get; private set; }

    public World()
    {
        Blocks = new WorldBlocks(this);
        Chunks = new WorldChunks(this);
    }

    public Material ChunkMaterial;

    public void Start()
    {
        for(int cx = -10; cx < 10; ++cx)
            for(int cy = -10; cy < 10; ++cy)
            {
                Chunk chunk = new Chunk(this, new BlockPos(cx*Constants.ChunkSize, 0, cy*Constants.ChunkSize));
                for(int x = 0; x < Constants.ChunkSize; ++x)
                    for(int z = 0; z < Constants.ChunkSize; ++z)
                        chunk.Set(new BlockPos(x,0,z)+chunk.Position, Block.Grass);

                for(int x = 7; x < 12; ++x)
                    for(int z = 6; z < 11; ++z)
                        chunk.Set(new BlockPos(x,1,z)+chunk.Position, Block.Grass);

                Chunks.Set(chunk.Position, chunk);
                SpawnChunkGameObject(chunk);
            }
    }
    
    GameObject SpawnChunkGameObject(Chunk chunk)
    {
        GameObject obj = new GameObject();
        obj.name = "Chunk " + chunk.Position.ToString();
        obj.AddComponent<MeshFilter>().mesh = ChunkMeshBuilder.Build(chunk);
        obj.AddComponent<MeshRenderer>().material = ChunkMaterial;
        obj.transform.localPosition = new Vector3(
            chunk.Position.x * Constants.BlockSize.x,
            chunk.Position.y * Constants.BlockSize.y,
            chunk.Position.z * Constants.BlockSize.z);
        obj.transform.SetParent(this.transform);
        return obj;
    }
}
