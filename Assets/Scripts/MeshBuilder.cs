using UnityEngine;
using System.Collections;
using System.Collections.Generic;

public class MeshBuilder
{
    public readonly List<Vector3> vertices = new List<Vector3>();
    public readonly List<Vector3> normals = new List<Vector3>();
    public readonly List<Color> colors = new List<Color>();
    public readonly List<int> indices = new List<int>();

    public MeshBuilder()
    {
        
    }

    public int AddVertex(Vector3 position, Vector3 normal, Color color)
    {
        vertices.Add(position);
        normals.Add(normal);
        colors.Add(color);
        return vertices.Count - 1;
    }

    /// <summary>
    /// Adds a quad joining the last 4 added vertices.
    /// </summary>
    public void AddQuad()
    {
        int vertexCount = vertices.Count;
        indices.Add(vertexCount - 1);
        indices.Add(vertexCount - 2);
        indices.Add(vertexCount - 4);
        indices.Add(vertexCount - 4);
        indices.Add(vertexCount - 2);
        indices.Add(vertexCount - 3);
    }
    
    /// <summary>
    /// Generates a Mesh from the data in this class.
    /// </summary>
    public Mesh Build()
    {
        Mesh mesh = new Mesh();
        mesh.vertices = vertices.ToArray();
        mesh.normals = normals.ToArray();
        mesh.colors = colors.ToArray();
        mesh.triangles = indices.ToArray();
        return mesh;        
    }
}
